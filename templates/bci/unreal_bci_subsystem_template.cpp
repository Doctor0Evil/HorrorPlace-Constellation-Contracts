// Template: Unreal BCI feature subsystem and HorrorDirector-style mapping for Horror$Place.

#include "unreal_bci_subsystem_template.h"

#include "Engine/Engine.h"
#include "Engine/World.h"
#include "Misc/FileHelper.h"
#include "Misc/Paths.h"
#include "Sockets.h"
#include "SocketSubsystem.h"
#include "HAL/RunnableThread.h"
#include "Json.h"
#include "JsonUtilities.h"

namespace
{
    FString ReadLineFromSocket(FSocket* Socket)
    {
        if (!Socket)
        {
            return FString();
        }

        TArray<uint8> Buffer;
        Buffer.SetNumUninitialized(4096);

        int32 BytesRead = 0;
        if (!Socket->Recv(Buffer.GetData(), Buffer.Num(), BytesRead, ESocketReceiveFlags::None) || BytesRead <= 0)
        {
            return FString();
        }

        FString Result;
        Result.AppendChars(reinterpret_cast<TCHAR*>(Buffer.GetData()), BytesRead / sizeof(TCHAR));
        return Result;
    }
}

// --------------------------- UEEGFeatureSubsystem ---------------------------

UEEGFeatureSubsystem::UEEGFeatureSubsystem()
    : ConnectionConfig(nullptr)
    , bHasFeatures(false)
{
}

void UEEGFeatureSubsystem::Initialize(FSubsystemCollectionBase& Collection)
{
    Super::Initialize(Collection);

    if (GEngine)
    {
        TickHandle = GEngine->OnWorldTickStart().AddUObject(this, &UEEGFeatureSubsystem::Tick);
    }
}

void UEEGFeatureSubsystem::Deinitialize()
{
    if (GEngine && TickHandle.IsValid())
    {
        GEngine->OnWorldTickStart().Remove(TickHandle);
    }

    Super::Deinitialize();
}

void UEEGFeatureSubsystem::Configure(UBCIConnectionConfig* InConfig)
{
    ConnectionConfig = InConfig;
}

bool UEEGFeatureSubsystem::HasValidFeatures() const
{
    return bHasFeatures;
}

FEEGFeatures UEEGFeatureSubsystem::GetLatestFeatures() const
{
    return LatestFeatures;
}

void UEEGFeatureSubsystem::Tick(float DeltaSeconds)
{
    if (!ConnectionConfig)
    {
        return;
    }

    switch (ConnectionConfig->Source)
    {
    case EBCIDataSource::LiveFeatureServer:
        TickLive(DeltaSeconds);
        break;
    case EBCIDataSource::ReplayFile:
        TickReplay(DeltaSeconds);
        break;
    default:
        break;
    }
}

void UEEGFeatureSubsystem::TickLive(float DeltaSeconds)
{
    static FSocket* Socket = nullptr;

    if (!Socket)
    {
        ISocketSubsystem* SocketSubsystem = ISocketSubsystem::Get(PLATFORM_SOCKETSUBSYSTEM);
        if (!SocketSubsystem)
        {
            return;
        }

        Socket = SocketSubsystem->CreateSocket(NAME_Stream, TEXT("BCI_LiveSocket"), false);

        FIPv4Address Addr;
        FIPv4Address::Parse(ConnectionConfig->Host, Addr);
        TSharedRef<FInternetAddr> InternetAddr = SocketSubsystem->CreateInternetAddr();
        InternetAddr->SetIp(Addr.Value);
        InternetAddr->SetPort(ConnectionConfig->Port);

        bool bConnected = Socket->Connect(*InternetAddr);
        if (!bConnected)
        {
            SocketSubsystem->DestroySocket(Socket);
            Socket = nullptr;
            return;
        }

        Socket->SetNonBlocking(true);
    }

    if (!Socket)
    {
        return;
    }

    uint32 PendingSize = 0;
    if (!Socket->HasPendingData(PendingSize) || PendingSize == 0)
    {
        return;
    }

    TArray<uint8> Buffer;
    Buffer.SetNumUninitialized(FMath::Min<int32>(PendingSize, 4096));

    int32 BytesRead = 0;
    if (!Socket->Recv(Buffer.GetData(), Buffer.Num(), BytesRead, ESocketReceiveFlags::None) || BytesRead <= 0)
    {
        return;
    }

    FString JsonLine = FString(UTF8_TO_TCHAR(reinterpret_cast<const char*>(Buffer.GetData())));
    JsonLine.TrimStartAndEndInline();
    if (!JsonLine.IsEmpty())
    {
        UpdateFromJsonLine(JsonLine);
    }
}

void UEEGFeatureSubsystem::TickReplay(float DeltaSeconds)
{
    static TArray<FString> ReplayLines;
    static int32 ReplayIndex = 0;
    static bool bInitialized = false;

    if (!bInitialized)
    {
        bInitialized = true;

        if (ConnectionConfig->ReplayFilePath.IsEmpty())
        {
            return;
        }

        FString FullPath = FPaths::ConvertRelativePathToFull(ConnectionConfig->ReplayFilePath);
        FString FileContents;
        if (FFileHelper::LoadFileToString(FileContents, *FullPath))
        {
            FileContents.ParseIntoArrayLines(ReplayLines, false);
        }
    }

    if (ReplayLines.Num() == 0 || ReplayIndex >= ReplayLines.Num())
    {
        return;
    }

    const FString& Line = ReplayLines[ReplayIndex++];
    FString TrimmedLine = Line;
    TrimmedLine.TrimStartAndEndInline();

    if (!TrimmedLine.IsEmpty())
    {
        UpdateFromJsonLine(TrimmedLine);
    }
}

void UEEGFeatureSubsystem::UpdateFromJsonLine(const FString& JsonLine)
{
    TSharedPtr<FJsonObject> RootObject;
    TSharedRef<TJsonReader<>> Reader = TJsonReaderFactory<>::Create(JsonLine);

    if (!FJsonSerializer::Deserialize(Reader, RootObject) || !RootObject.IsValid())
    {
        return;
    }

    FEEGFeatures Parsed;

    const TSharedPtr<FJsonObject>* MetaObjPtr = nullptr;
    if (RootObject->TryGetObjectField(TEXT("meta"), MetaObjPtr))
    {
        const TSharedPtr<FJsonObject>& MetaObj = *MetaObjPtr;
        MetaObj->TryGetStringField(TEXT("session_id"), Parsed.Meta.SessionId);
        MetaObj->TryGetNumberField(TEXT("timestamp"), Parsed.Meta.Timestamp);
        MetaObj->TryGetStringField(TEXT("device_id"), Parsed.Meta.DeviceId);
        MetaObj->TryGetStringField(TEXT("schema_id"), Parsed.Meta.SchemaId);
        MetaObj->TryGetStringField(TEXT("version"), Parsed.Meta.Version);
    }

    const TSharedPtr<FJsonObject>* BandsObjPtr = nullptr;
    if (RootObject->TryGetObjectField(TEXT("bands"), BandsObjPtr))
    {
        const TSharedPtr<FJsonObject>& BandsObj = *BandsObjPtr;
        BandsObj->TryGetNumberField(TEXT("delta"), Parsed.Bands.Delta);
        BandsObj->TryGetNumberField(TEXT("theta"), Parsed.Bands.Theta);
        BandsObj->TryGetNumberField(TEXT("alpha"), Parsed.Bands.Alpha);
        BandsObj->TryGetNumberField(TEXT("beta"), Parsed.Bands.Beta);
        BandsObj->TryGetNumberField(TEXT("gamma"), Parsed.Bands.Gamma);
    }

    const TSharedPtr<FJsonObject>* CompositeObjPtr = nullptr;
    if (RootObject->TryGetObjectField(TEXT("composite"), CompositeObjPtr))
    {
        const TSharedPtr<FJsonObject>& CompositeObj = *CompositeObjPtr;
        CompositeObj->TryGetNumberField(TEXT("stress"), Parsed.Composite.Stress);
        CompositeObj->TryGetNumberField(TEXT("focus"), Parsed.Composite.Focus);
        CompositeObj->TryGetNumberField(TEXT("fatigue"), Parsed.Composite.Fatigue);
    }

    const TSharedPtr<FJsonObject>* HorrorContextObjPtr = nullptr;
    if (RootObject->TryGetObjectField(TEXT("horror_context"), HorrorContextObjPtr))
    {
        const TSharedPtr<FJsonObject>& HCObj = *HorrorContextObjPtr;
        HCObj->TryGetNumberField(TEXT("CIC"), Parsed.HorrorContext.CIC);
        HCObj->TryGetNumberField(TEXT("MDI"), Parsed.HorrorContext.MDI);
        HCObj->TryGetNumberField(TEXT("AOS"), Parsed.HorrorContext.AOS);
        HCObj->TryGetNumberField(TEXT("DET"), Parsed.HorrorContext.DET);
        HCObj->TryGetNumberField(TEXT("HVF"), Parsed.HorrorContext.HVF);
        HCObj->TryGetNumberField(TEXT("LSG"), Parsed.HorrorContext.LSG);
        HCObj->TryGetNumberField(TEXT("SHCI"), Parsed.HorrorContext.SHCI);
        HCObj->TryGetNumberField(TEXT("UEC"), Parsed.HorrorContext.UEC);
        HCObj->TryGetNumberField(TEXT("ARR"), Parsed.HorrorContext.ARR);
    }

    LatestFeatures = Parsed;
    bHasFeatures = true;
}

// ----------------------- UBCIHorrorDirectorSubsystem -----------------------

UBCIHorrorDirectorSubsystem::UBCIHorrorDirectorSubsystem()
    : ConnectionConfig(nullptr)
    , Tension(0.0f)
    , EnemySpawnMultiplier(1.0f)
    , StressToTensionSlope(1.0f)
    , CicToTensionSlope(0.5f)
    , BaselineTension(0.1f)
    , TensionToSpawnSlope(1.5f)
    , BaselineSpawnMultiplier(1.0f)
{
}

void UBCIHorrorDirectorSubsystem::Initialize(FSubsystemCollectionBase& Collection)
{
    Super::Initialize(Collection);

    if (GEngine)
    {
        TickHandle = GEngine->OnWorldTickStart().AddUObject(this, &UBCIHorrorDirectorSubsystem::Tick);
    }
}

void UBCIHorrorDirectorSubsystem::Deinitialize()
{
    if (GEngine && TickHandle.IsValid())
    {
        GEngine->OnWorldTickStart().Remove(TickHandle);
    }

    Super::Deinitialize();
}

void UBCIHorrorDirectorSubsystem::Configure(UBCIConnectionConfig* InConfig)
{
    ConnectionConfig = InConfig;
}

void UBCIHorrorDirectorSubsystem::SetMappingParameters(
    float InStressToTensionSlope,
    float InCicToTensionSlope,
    float InBaselineTension,
    float InTensionToSpawnSlope,
    float InBaselineSpawnMultiplier)
{
    StressToTensionSlope = InStressToTensionSlope;
    CicToTensionSlope = InCicToTensionSlope;
    BaselineTension = InBaselineTension;
    TensionToSpawnSlope = InTensionToSpawnSlope;
    BaselineSpawnMultiplier = InBaselineSpawnMultiplier;
}

void UBCIHorrorDirectorSubsystem::Tick(float DeltaSeconds)
{
    if (!ConnectionConfig)
    {
        return;
    }

    UEEGFeatureSubsystem* EEGSubsystem = GEngine ? GEngine->GetEngineSubsystem<UEEGFeatureSubsystem>() : nullptr;
    if (!EEGSubsystem || !EEGSubsystem->HasValidFeatures())
    {
        return;
    }

    const FEEGFeatures Features = EEGSubsystem->GetLatestFeatures();
    ApplyMapping(Features, DeltaSeconds);
}

void UBCIHorrorDirectorSubsystem::ApplyMapping(const FEEGFeatures& Features, float DeltaSeconds)
{
    const float Stress = static_cast<float>(Features.Composite.Stress);
    const float CIC = static_cast<float>(Features.HorrorContext.CIC);

    const float NewTension = ComputeClampedTension(Stress, CIC, DeltaSeconds);
    const float MaxTension = ConnectionConfig ? ConnectionConfig->MaxTension : 1.0f;
    Tension = FMath::Clamp(NewTension, 0.0f, MaxTension);

    const float RawSpawnMultiplier = BaselineSpawnMultiplier + TensionToSpawnSlope * Tension;
    const float MaxSpawnMultiplier = ConnectionConfig ? ConnectionConfig->MaxSpawnMultiplier : 5.0f;
    EnemySpawnMultiplier = FMath::Clamp(RawSpawnMultiplier, 0.0f, MaxSpawnMultiplier);
}

float UBCIHorrorDirectorSubsystem::ComputeClampedTension(float Stress, float CIC, float DeltaSeconds) const
{
    const float Target = BaselineTension
        + StressToTensionSlope * Stress
        + CicToTensionSlope * CIC;

    const float ClampedTarget = FMath::Clamp(Target, 0.0f, 1.0f);

    const float MaxRateOfChange = 1.0f * DeltaSeconds;
    const float Delta = FMath::Clamp(ClampedTarget - Tension, -MaxRateOfChange, MaxRateOfChange);

    return Tension + Delta;
}
