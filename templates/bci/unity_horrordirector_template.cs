// Template: Unity EEGFeatureService and HorrorDirector integration for Horror$Place.
//
// Behavior:
// - EEGFeatureService connects to a feature server (live or replay) and exposes EEGFeatures.
// - HorrorDirector reads EEGFeatures, applies mapping and BCI intensity policies,
//   and drives 1–2 simple outputs (e.g., post-process weight and enemy spawn multiplier).
//
// This template intentionally avoids hardware details. It assumes a TCP NDJSON feature server
// producing EEGFeatureContractv1-compatible payloads.

using System;
using System.Collections;
using System.Collections.Generic;
using System.Net.Sockets;
using System.Text;
using System.Threading;
using Newtonsoft.Json;
using UnityEngine;

namespace HorrorPlace.BCI.Unity
{
    [Serializable]
    public class EEGFeatures
    {
        [Serializable]
        public class Meta
        {
            public string session_id;
            public double timestamp;
            public string device_id;
            public string schema_id;
            public string version;
        }

        [Serializable]
        public class Bands
        {
            public double delta;
            public double theta;
            public double alpha;
            public double beta;
            public double gamma;
        }

        [Serializable]
        public class Composite
        {
            public double stress;
            public double focus;
            public double fatigue;
        }

        [Serializable]
        public class HorrorContext
        {
            public double CIC;
            public double MDI;
            public double AOS;
            public double DET;
            public double HVF;
            public double LSG;
            public double SHCI;
            public double UEC;
            public double ARR;
        }

        public Meta meta;
        public Bands bands;
        public Composite composite;
        public HorrorContext horror_context;
    }

    [CreateAssetMenu(fileName = "BCIConnectionConfig", menuName = "HorrorPlace/BCI/ConnectionConfig")]
    public class BCIConnectionConfig : ScriptableObject
    {
        public enum DataSource
        {
            LiveFeatureServer,
            ReplayFile
        }

        public DataSource Source = DataSource.LiveFeatureServer;

        [Header("Live feature server")]
        public string Host = "127.0.0.1";
        public int Port = 7777;

        [Header("Replay")]
        public TextAsset ReplayNdjson;

        [Header("BCI Policy Caps")]
        [Range(0f, 1f)] public float MaxTension = 0.8f;
        [Range(0f, 5f)] public float MaxSpawnMultiplier = 3.0f;
    }

    public sealed class EEGFeatureService : MonoBehaviour
    {
        public static EEGFeatureService Instance { get; private set; }

        [SerializeField]
        private BCIConnectionConfig _config;

        private readonly object _lock = new object();
        private EEGFeatures _latestFeatures;

        private Thread _networkThread;
        private volatile bool _running;

        private void Awake()
        {
            if (Instance != null && Instance != this)
            {
                Destroy(gameObject);
                return;
            }

            Instance = this;
            DontDestroyOnLoad(gameObject);
        }

        private void OnEnable()
        {
            StartService();
        }

        private void OnDisable()
        {
            StopService();
        }

        public EEGFeatures GetLatestFeatures()
        {
            lock (_lock)
            {
                return _latestFeatures;
            }
        }

        private void StartService()
        {
            if (_config == null)
            {
                Debug.LogError("[EEGFeatureService] No connection config assigned.");
                return;
            }

            if (_config.Source == BCIConnectionConfig.DataSource.LiveFeatureServer)
            {
                StartNetworkThread();
            }
            else
            {
                StartCoroutine(ReplayCoroutine());
            }
        }

        private void StopService()
        {
            _running = false;

            if (_networkThread != null)
            {
                try
                {
                    _networkThread.Join(500);
                }
                catch (Exception)
                {
                }

                _networkThread = null;
            }
        }

        private void StartNetworkThread()
        {
            if (_networkThread != null)
            {
                return;
            }

            _running = true;
            _networkThread = new Thread(NetworkLoop) { IsBackground = true };
            _networkThread.Start();
        }

        private void NetworkLoop()
        {
            try
            {
                using (var client = new TcpClient())
                {
                    client.Connect(_config.Host, _config.Port);
                    using (var stream = client.GetStream())
                    {
                        var buffer = new byte[4096];
                        var sb = new StringBuilder();

                        while (_running)
                        {
                            if (!stream.DataAvailable)
                            {
                                Thread.Sleep(5);
                                continue;
                            }

                            int bytesRead = stream.Read(buffer, 0, buffer.Length);
                            if (bytesRead <= 0)
                            {
                                break;
                            }

                            sb.Append(Encoding.UTF8.GetString(buffer, 0, bytesRead));
                            ProcessBufferedLines(sb);
                        }
                    }
                }
            }
            catch (Exception ex)
            {
                Debug.LogError($"[EEGFeatureService] NetworkLoop error: {ex.Message}");
            }
        }

        private void ProcessBufferedLines(StringBuilder sb)
        {
            while (true)
            {
                string current = sb.ToString();
                int newlineIndex = current.IndexOf('\n');
                if (newlineIndex < 0)
                {
                    break;
                }

                string line = current.Substring(0, newlineIndex).Trim();
                sb.Remove(0, newlineIndex + 1);

                if (string.IsNullOrEmpty(line))
                {
                    continue;
                }

                TryUpdateFeatures(line);
            }
        }

        private void TryUpdateFeatures(string jsonLine)
        {
            try
            {
                var features = JsonConvert.DeserializeObject<EEGFeatures>(jsonLine);
                if (features == null)
                {
                    return;
                }

                lock (_lock)
                {
                    _latestFeatures = features;
                }
            }
            catch (Exception ex)
            {
                Debug.LogWarning($"[EEGFeatureService] Failed to deserialize EEGFeatures: {ex.Message}");
            }
        }

        private IEnumerator ReplayCoroutine()
        {
            if (_config.ReplayNdjson == null)
            {
                Debug.LogError("[EEGFeatureService] Replay source not configured.");
                yield break;
            }

            var lines = _config.ReplayNdjson.text.Split(new[] { '\n', '\r' }, StringSplitOptions.RemoveEmptyEntries);
            foreach (var line in lines)
            {
                TryUpdateFeatures(line);
                yield return null;
            }

            Debug.Log("[EEGFeatureService] Replay completed.");
        }
    }

    public sealed class HorrorDirector : MonoBehaviour
    {
        [Header("References")]
        public EEGFeatureService FeatureService;
        public BCIConnectionConfig BciConfig;

        [Header("Outputs")]
        [Range(0f, 1f)] public float Tension;
        [Range(0f, 5f)] public float EnemySpawnMultiplier = 1f;

        [Header("Mapping Parameters")]
        [Range(0f, 1f)] public float StressToTensionSlope = 1f;
        [Range(0f, 1f)] public float CicToTensionSlope = 0.5f;
        [Range(0f, 1f)] public float BaselineTension = 0.1f;

        [Range(0f, 3f)] public float TensionToSpawnSlope = 1.5f;
        [Range(0f, 5f)] public float BaselineSpawnMultiplier = 1f;

        private void Reset()
        {
            if (FeatureService == null)
            {
                FeatureService = EEGFeatureService.Instance;
            }
        }

        private void Update()
        {
            if (FeatureService == null || BciConfig == null)
            {
                return;
            }

            var features = FeatureService.GetLatestFeatures();
            if (features == null || features.composite == null || features.horror_context == null)
            {
                return;
            }

            float stress = (float)features.composite.stress;
            float cic = (float)features.horror_context.CIC;

            float tension = BaselineTension
                            + StressToTensionSlope * stress
                            + CicToTensionSlope * cic;

            tension = Mathf.Clamp01(tension);
            tension = Mathf.Min(tension, BciConfig.MaxTension);

            float spawnMultiplier = BaselineSpawnMultiplier + TensionToSpawnSlope * tension;
            spawnMultiplier = Mathf.Clamp(spawnMultiplier, 0f, BciConfig.MaxSpawnMultiplier);

            Tension = tension;
            EnemySpawnMultiplier = spawnMultiplier;

            ApplyOutputs();
        }

        private void ApplyOutputs()
        {
            var ppv = FindObjectOfType<UnityEngine.Rendering.Volume>();
            if (ppv != null)
            {
                ppv.weight = Mathf.Lerp(ppv.weight, Tension, Time.deltaTime * 2f);
            }

            var spawners = FindObjectsOfType<SimpleBCIEnemySpawner>();
            foreach (var spawner in spawners)
            {
                spawner.SetBCIMultiplier(EnemySpawnMultiplier);
            }
        }
    }

    public sealed class SimpleBCIEnemySpawner : MonoBehaviour
    {
        [Header("Base Settings")]
        public float BaseSpawnInterval = 5f;

        private float _bciMultiplier = 1f;
        private float _nextSpawnTime;

        private void Update()
        {
            if (Time.time >= _nextSpawnTime)
            {
                SpawnEnemy();
                var interval = Mathf.Max(0.2f, BaseSpawnInterval / Mathf.Max(0.1f, _bciMultiplier));
                _nextSpawnTime = Time.time + interval;
            }
        }

        public void SetBCIMultiplier(float multiplier)
        {
            _bciMultiplier = Mathf.Max(0.1f, multiplier);
        }

        private void SpawnEnemy()
        {
            // Implement your actual enemy spawn logic here.
            Debug.Log($"[SimpleBCIEnemySpawner] Spawned enemy with BCI multiplier = {_bciMultiplier:F2}");
        }
    }
}
