// Template: EEG device driver implementing IEEGDevice pattern for Horror$Place.
// This file is a skeleton; implementations must remain aligned with BCI-Dev-Guide and EEGFeatureContractv1.

using System;
using System.Net.Sockets;
using System.Text;
using System.Threading;
using System.Threading.Tasks;
using System.Collections.Generic;
using Newtonsoft.Json;

namespace HorrorPlace.BCI.Devices
{
    // Represents the canonical feature snapshot derived from EEGFeatureContractv1.
    // Fields should mirror the schema but remain minimal here to keep the template focused.
    public sealed class EEGFeatures
    {
        // Core metadata
        public string SessionId { get; set; }
        public DateTime TimestampUtc { get; set; }

        // Example composite features
        public double Stress { get; set; }
        public double Focus { get; set; }

        // Example horror context metrics
        public double CIC { get; set; }
        public double MDI { get; set; }
        public double DET { get; set; }
        public double Tension { get; set; }

        // Extend with additional properties only when reflected in the schema and dev guide.
    }

    // Minimal logical device configuration entry loaded from a registry (for example, EEGDeviceRegistry.json).
    public sealed class EEGDeviceConfig
    {
        public string LogicalDeviceId { get; set; }
        public string Backend { get; set; } // "brainflow" or "lsl"
        public string Host { get; set; }    // Feature server host for high-level connections.
        public int Port { get; set; }       // Feature server port for high-level connections.

        // Backend-specific parameters, if needed.
        public Dictionary<string, string> Parameters { get; set; } = new Dictionary<string, string>();
    }

    // Interface for EEG device drivers. Higher-level systems use this interface to consume EEGFeatures.
    public interface IEEGDevice : IDisposable
    {
        EEGDeviceConfig Config { get; }

        bool IsConnected { get; }

        Task ConnectAsync(CancellationToken cancellationToken = default);

        Task DisconnectAsync(CancellationToken cancellationToken = default);

        // Returns the latest available EEGFeatures snapshot, or null if none are available yet.
        EEGFeatures GetLatestFeatures();
    }

    // Template implementation that connects to a feature server streaming EEGFeatureContract-compatible NDJSON over TCP.
    public sealed class TcpFeatureServerEEGDevice : IEEGDevice
    {
        private readonly object _lock = new object();
        private readonly EEGDeviceConfig _config;

        private TcpClient _client;
        private NetworkStream _stream;
        private CancellationTokenSource _receiveCts;
        private Task _receiveTask;

        private EEGFeatures _latestFeatures;

        public TcpFeatureServerEEGDevice(EEGDeviceConfig config)
        {
            _config = config ?? throw new ArgumentNullException(nameof(config));
        }

        public EEGDeviceConfig Config => _config;

        public bool IsConnected
        {
            get
            {
                lock (_lock)
                {
                    return _client != null && _client.Connected;
                }
            }
        }

        public async Task ConnectAsync(CancellationToken cancellationToken = default)
        {
            lock (_lock)
            {
                if (_client != null)
                {
                    throw new InvalidOperationException("Device is already connected.");
                }

                _client = new TcpClient();
            }

            await _client.ConnectAsync(_config.Host, _config.Port).ConfigureAwait(false);

            lock (_lock)
            {
                _stream = _client.GetStream();
                _receiveCts = new CancellationTokenSource();
                _receiveTask = Task.Run(() => ReceiveLoopAsync(_receiveCts.Token), _receiveCts.Token);
            }
        }

        public async Task DisconnectAsync(CancellationToken cancellationToken = default)
        {
            CancellationTokenSource cts;
            Task receiveTask;

            lock (_lock)
            {
                cts = _receiveCts;
                receiveTask = _receiveTask;

                _receiveCts = null;
                _receiveTask = null;
            }

            if (cts != null)
            {
                cts.Cancel();
            }

            if (receiveTask != null)
            {
                try
                {
                    await receiveTask.ConfigureAwait(false);
                }
                catch (OperationCanceledException)
                {
                    // Expected on cancellation.
                }
            }

            lock (_lock)
            {
                if (_stream != null)
                {
                    _stream.Dispose();
                    _stream = null;
                }

                if (_client != null)
                {
                    _client.Close();
                    _client = null;
                }
            }
        }

        public EEGFeatures GetLatestFeatures()
        {
            lock (_lock)
            {
                return _latestFeatures;
            }
        }

        private async Task ReceiveLoopAsync(CancellationToken cancellationToken)
        {
            var buffer = new byte[4096];
            var sb = new StringBuilder();

            while (!cancellationToken.IsCancellationRequested)
            {
                int bytesRead;
                try
                {
                    bytesRead = await _stream.ReadAsync(buffer, 0, buffer.Length, cancellationToken).ConfigureAwait(false);
                }
                catch (OperationCanceledException)
                {
                    break;
                }
                catch
                {
                    // Network error; break and let caller decide what to do.
                    break;
                }

                if (bytesRead <= 0)
                {
                    // Disconnected.
                    break;
                }

                sb.Append(Encoding.UTF8.GetString(buffer, 0, bytesRead));

                while (true)
                {
                    var newlineIndex = sb.ToString().IndexOf('\n');
                    if (newlineIndex < 0)
                    {
                        break;
                    }

                    var line = sb.ToString(0, newlineIndex).Trim();
                    sb.Remove(0, newlineIndex + 1);

                    if (string.IsNullOrEmpty(line))
                    {
                        continue;
                    }

                    // Each line should be an EEGFeatureContract-compatible JSON object.
                    TryUpdateFeatures(line);
                }
            }
        }

        private void TryUpdateFeatures(string jsonLine)
        {
            try
            {
                // This assumes that the JSON payload can be mapped to EEGFeatures.
                // In a real implementation, you would deserialize into a schema-aligned DTO
                // and then map that DTO to EEGFeatures.
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
            catch
            {
                // Malformed line; ignore or log through the configured telemetry pipeline.
            }
        }

        public void Dispose()
        {
            _ = DisconnectAsync();
        }
    }
}
