// Template: Unreal BCI feature subsystem and HorrorDirector-style mapping for Horror$Place.
//
// Behavior:
// - UEEGFeatureSubsystem connects to a feature server (live) or replay source and exposes FEEGFeatures.
// - UBCIHorrorDirectorSubsystem reads FEEGFeatures, applies mapping and BCI intensity policies,
//   and exposes tension and enemy spawn multiplier for game systems and Blueprints.

#pragma once

#include "CoreMinimal.h"
#include "Engine/EngineSubsystem.h"
#include "Engine/DataAsset.h"
#include "unreal_bci_subsystem_template.generated.h"

UENUM(BlueprintType)
enum class EBCIDataSource : uint8
{
    LiveFeatureServer UMETA(DisplayName = "Live Feature Server"),
    ReplayFile        UMETA(DisplayName = "Replay File")
};

UCLASS(BlueprintType)
class UBCIConnectionConfig : public UDataAsset
{
    GENERATED_BODY()

public:
    UPROPERTY(EditAnywhere, BlueprintReadOnly, Category = "BCI")
    EBCIDataSource Source = EBCIDataSource::LiveFeatureServer;

    UPROPERTY(EditAnywhere, BlueprintReadOnly, Category = "BCI|Live")
    FString Host = TEXT("127.0.0.1");

    UPROPERTY(EditAnywhere, BlueprintReadOnly, Category = "BCI|Live")
    int32 Port = 7777;

    UPROPERTY(EditAnywhere, BlueprintReadOnly, Category = "BCI|Replay")
    FString ReplayFilePath;

    UPROPERTY(EditAnywhere, BlueprintReadOnly, Category = "BCI|Policy", meta = (ClampMin = "0.0", ClampMax = "1.0"))
    float MaxTension = 0.8f;

    UPROPERTY(EditAnywhere, BlueprintReadOnly, Category = "BCI|Policy", meta = (ClampMin = "0.0", ClampMax = "5.0"))
    float MaxSpawnMultiplier = 3.0f;
};

USTRUCT(BlueprintType)
struct FEEGMeta
{
    GENERATED_BODY()

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "EEG")
    FString SessionId;

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "EEG")
    double Timestamp = 0.0;

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "EEG")
    FString DeviceId;

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "EEG")
    FString SchemaId;

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "EEG")
    FString Version;
};

USTRUCT(BlueprintType)
struct FEEGBands
{
    GENERATED_BODY()

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "EEG")
    double Delta = 0.0;

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "EEG")
    double Theta = 0.0;

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "EEG")
    double Alpha = 0.0;

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "EEG")
    double Beta = 0.0;

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "EEG")
    double Gamma = 0.0;
};

USTRUCT(BlueprintType)
struct FEEGComposite
{
    GENERATED_BODY()

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "EEG")
    double Stress = 0.0;

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "EEG")
    double Focus = 0.0;

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "EEG")
    double Fatigue = 0.0;
};

USTRUCT(BlueprintType)
struct FEEGHorrorContext
{
    GENERATED_BODY()

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "EEG")
    double CIC = 0.0;

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "EEG")
    double MDI = 0.0;

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "EEG")
    double AOS = 0.0;

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "EEG")
    double DET = 0.0;

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "EEG")
    double HVF = 0.0;

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "EEG")
    double LSG = 0.0;

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "EEG")
    double SHCI = 0.0;

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "EEG")
    double UEC = 0.0;

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "EEG")
    double ARR = 0.0;
};

USTRUCT(BlueprintType)
struct FEEGFeatures
{
    GENERATED_BODY()

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "EEG")
    FEEGMeta Meta;

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "EEG")
    FEEGBands Bands;

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "EEG")
    FEEGComposite Composite;

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "EEG")
    FEEGHorrorContext HorrorContext;
};

UCLASS()
class UEEGFeatureSubsystem : public UEngineSubsystem
{
    GENERATED_BODY()

public:
    UEEGFeatureSubsystem();

    virtual void Initialize(FSubsystemCollectionBase& Collection) override;
    virtual void Deinitialize() override;

    UFUNCTION(BlueprintCallable, Category = "BCI")
    void Configure(UBCIConnectionConfig* InConfig);

    UFUNCTION(BlueprintCallable, Category = "BCI")
    bool HasValidFeatures() const;

    UFUNCTION(BlueprintCallable, Category = "BCI")
    FEEGFeatures GetLatestFeatures() const;

private:
    UPROPERTY()
    UBCIConnectionConfig* ConnectionConfig;

    FEEGFeatures LatestFeatures;
    bool bHasFeatures;

    FDelegateHandle TickHandle;

    void Tick(float DeltaSeconds);

    void TickLive(float DeltaSeconds);
    void TickReplay(float DeltaSeconds);

    void UpdateFromJsonLine(const FString& JsonLine);
};

UCLASS()
class UBCIHorrorDirectorSubsystem : public UEngineSubsystem
{
    GENERATED_BODY()

public:
    UBCIHorrorDirectorSubsystem();

    virtual void Initialize(FSubsystemCollectionBase& Collection) override;
    virtual void Deinitialize() override;

    UFUNCTION(BlueprintCallable, Category = "BCI")
    void Configure(UBCIConnectionConfig* InConfig);

    UFUNCTION(BlueprintCallable, Category = "BCI")
    float GetTension() const { return Tension; }

    UFUNCTION(BlueprintCallable, Category = "BCI")
    float GetEnemySpawnMultiplier() const { return EnemySpawnMultiplier; }

    UFUNCTION(BlueprintCallable, Category = "BCI")
    void SetMappingParameters(
        float InStressToTensionSlope,
        float InCicToTensionSlope,
        float InBaselineTension,
        float InTensionToSpawnSlope,
        float InBaselineSpawnMultiplier);

private:
    UPROPERTY()
    UBCIConnectionConfig* ConnectionConfig;

    float Tension;
    float EnemySpawnMultiplier;

    float StressToTensionSlope;
    float CicToTensionSlope;
    float BaselineTension;

    float TensionToSpawnSlope;
    float BaselineSpawnMultiplier;

    FDelegateHandle TickHandle;

    void Tick(float DeltaSeconds);

    void ApplyMapping(const FEEGFeatures& Features, float DeltaSeconds);

    float ComputeClampedTension(float Stress, float CIC, float DeltaSeconds) const;
};
