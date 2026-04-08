#!/usr/bin/env python3
"""
Template: EEG feature server for Horror$Place BCI stack.

Behavior:
- Loads a logical device config from a registry (e.g., EEGDeviceRegistry.json).
- Connects to a BrainFlow board (or LSL stream) using the config.
- Runs the EEGCanonicalV1 feature extraction pipeline.
- Emits EEGFeatureContractv1-compatible NDJSON over TCP.

This template is intentionally minimal. Implementors must:
- Keep field names aligned with EEGFeatureContractv1.
- Avoid logging raw EEG.
- Keep hardware details out of engine/game code.
"""

import argparse
import json
import socket
import threading
import time
from typing import Any, Dict, List, Optional

import numpy as np

try:
    from brainflow.board_shim import BoardShim, BrainFlowInputParams, BoardIds
except ImportError:
    BoardShim = None  # Placeholder for environments without BrainFlow.


# -------------------------------------------------------------------------
# Config and registry loading
# -------------------------------------------------------------------------


def load_device_registry(path: str) -> Dict[str, Any]:
    with open(path, "r", encoding="utf-8") as f:
        return json.load(f)


def get_logical_device(registry: Dict[str, Any], logical_device_id: str) -> Dict[str, Any]:
    devices = registry.get("devices", [])
    for device in devices:
        if device.get("logical_device_id") == logical_device_id:
            return device
    raise ValueError(f"Logical device '{logical_device_id}' not found in registry.")


# -------------------------------------------------------------------------
# BrainFlow / backend wiring
# -------------------------------------------------------------------------


class BrainFlowBackend:
    """
    Minimal BrainFlow backend wrapper.

    In a real implementation, you should:
    - Map logical device config to BrainFlowInputParams and board ID.
    - Handle connection errors and reconnection logic.
    - Perform proper cleanup on shutdown.
    """

    def __init__(self, device_config: Dict[str, Any]) -> None:
        if BoardShim is None:
            raise RuntimeError("BrainFlow is not available in this environment.")

        self.device_config = device_config
        self.params = BrainFlowInputParams()

        # Example: map configuration into BrainFlowInputParams.
        self.params.serial_port = device_config.get("serial_port", "")
        self.params.mac_address = device_config.get("mac_address", "")
        self.params.ip_address = device_config.get("ip_address", "")
        self.params.ip_port = int(device_config.get("ip_port", 0)) or 0

        board_id_value = device_config.get("brainflow_board_id", BoardIds.SYNTHETIC_BOARD.value)
        self.board_id = int(board_id_value)

        self.board = BoardShim(self.board_id, self.params)
        self.sampling_rate = BoardShim.get_sampling_rate(self.board_id)
        self.eeg_channels = BoardShim.get_eeg_channels(self.board_id)

    def start(self) -> None:
        self.board.prepare_session()
        self.board.start_stream()

    def stop(self) -> None:
        try:
            self.board.stop_stream()
        finally:
            self.board.release_session()

    def get_window(self, window_seconds: float) -> np.ndarray:
        length = int(window_seconds * self.sampling_rate)
        data = self.board.get_current_board_data(length)
        if data.size == 0:
            return np.empty((0, 0))
        return data[self.eeg_channels, :]


# -------------------------------------------------------------------------
# EEGCanonicalV1 feature extraction
# -------------------------------------------------------------------------


def compute_band_powers(eeg_window: np.ndarray, sampling_rate: float) -> Dict[str, float]:
    """
    Very simplified placeholder band power computation.

    Replace with:
    - Real filtering and PSD computation.
    - Canonical band definitions (delta, theta, alpha, beta, gamma).
    """
    if eeg_window.size == 0:
        return {
            "delta": 0.0,
            "theta": 0.0,
            "alpha": 0.0,
            "beta": 0.0,
            "gamma": 0.0,
        }

    # Placeholder: use variance as a crude energy proxy.
    channel_var = np.var(eeg_window, axis=1)
    total = float(np.sum(channel_var)) or 1.0

    # Split variance into pseudo-bands purely as a template.
    bands = {
        "delta": float(0.25 * total),
        "theta": float(0.20 * total),
        "alpha": float(0.20 * total),
        "beta": float(0.20 * total),
        "gamma": float(0.15 * total),
    }
    return bands


def compute_composite_features(bands: Dict[str, float]) -> Dict[str, float]:
    """
    Compute simple composite scores from band powers.

    Replace with your canonical EEGCanonicalV1 definitions.
    """
    alpha = bands.get("alpha", 0.0)
    beta = bands.get("beta", 0.0)
    theta = bands.get("theta", 0.0)
    delta = bands.get("delta", 0.0)

    total = alpha + beta + theta + delta or 1.0

    stress = float((beta + theta) / total)
    focus = float(alpha / total)
    fatigue = float(delta / total)

    return {
        "stress": stress,
        "focus": focus,
        "fatigue": fatigue,
    }


def compute_horror_context(composite: Dict[str, float], prev_state: Optional[Dict[str, float]] = None) -> Dict[str, float]:
    """
    Compute horror_context metrics from composite features.

    This is a placeholder that illustrates field names and normalization.
    Real implementations should use your validated formulas.
    """
    stress = composite.get("stress", 0.0)
    fatigue = composite.get("fatigue", 0.0)
    focus = composite.get("focus", 0.0)

    # Clamp to [0, 1].
    stress = max(0.0, min(1.0, stress))
    fatigue = max(0.0, min(1.0, fatigue))
    focus = max(0.0, min(1.0, focus))

    cic = float(0.5 * focus + 0.5 * (1.0 - fatigue))
    mdi = float(0.6 * stress + 0.4 * fatigue)
    aos = float(abs(stress - (prev_state or {}).get("stress", stress)))
    det = float((prev_state or {}).get("det", 0.0) + mdi * 0.01)

    cic = max(0.0, min(1.0, cic))
    mdi = max(0.0, min(1.0, mdi))
    aos = max(0.0, min(1.0, aos))
    det = max(0.0, min(1.0, det))

    hvf = float(0.5 * aos + 0.5 * stress)
    lsg = float(abs(cic - (prev_state or {}).get("cic", cic)))
    shci = float(0.5 * cic + 0.5 * (1.0 - aos))
    uec = float(stress * (1.0 - focus))
    arr = float(1.0 - mdi)

    return {
        "CIC": cic,
        "MDI": mdi,
        "AOS": aos,
        "DET": det,
        "HVF": hvf,
        "LSG": lsg,
        "SHCI": shci,
        "UEC": uec,
        "ARR": arr,
        "stress": stress,
        "fatigue": fatigue,
        "focus": focus,
    }


# -------------------------------------------------------------------------
# EEGFeatureContract JSON construction
# -------------------------------------------------------------------------


def build_eeg_feature_contract(
    device_config: Dict[str, Any],
    bands: Dict[str, float],
    composite: Dict[str, float],
    horror_context: Dict[str, float],
    session_id: str,
) -> Dict[str, Any]:
    """
    Build a minimal EEGFeatureContractv1-compatible payload.

    This must stay aligned with the JSON Schema in Horror.Place.
    """
    timestamp = time.time()
    payload = {
        "meta": {
            "session_id": session_id,
            "timestamp": timestamp,
            "device_id": device_config.get("logical_device_id", ""),
            "schema_id": "EEGFeatureContractv1",
            "version": "1.0.0",
        },
        "bands": {
            "delta": bands.get("delta", 0.0),
            "theta": bands.get("theta", 0.0),
            "alpha": bands.get("alpha", 0.0),
            "beta": bands.get("beta", 0.0),
            "gamma": bands.get("gamma", 0.0),
        },
        "composite": {
            "stress": composite.get("stress", 0.0),
            "focus": composite.get("focus", 0.0),
            "fatigue": composite.get("fatigue", 0.0),
        },
        "horror_context": {
            "CIC": horror_context.get("CIC", 0.0),
            "MDI": horror_context.get("MDI", 0.0),
            "AOS": horror_context.get("AOS", 0.0),
            "DET": horror_context.get("DET", 0.0),
            "HVF": horror_context.get("HVF", 0.0),
            "LSG": horror_context.get("LSG", 0.0),
            "SHCI": horror_context.get("SHCI", 0.0),
            "UEC": horror_context.get("UEC", 0.0),
            "ARR": horror_context.get("ARR", 0.0),
        },
        "debug": {
            "stress": horror_context.get("stress", 0.0),
            "fatigue": horror_context.get("fatigue", 0.0),
            "focus": horror_context.get("focus", 0.0),
        },
    }
    return payload


# -------------------------------------------------------------------------
# NDJSON TCP server
# -------------------------------------------------------------------------


class NDJSONServer:
    """
    Minimal single-client NDJSON TCP server.

    For production:
    - Add proper logging, error handling, and multi-client support.
    - Consider TLS if appropriate.
    """

    def __init__(self, host: str, port: int) -> None:
        self.host = host
        self.port = port
        self._server_socket: Optional[socket.socket] = None
        self._client_socket: Optional[socket.socket] = None
        self._lock = threading.Lock()

    def start(self) -> None:
        sock = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
        sock.bind((self.host, self.port))
        sock.listen(1)
        self._server_socket = sock
        print(f"[feature_server] Listening on {self.host}:{self.port} (NDJSON)")

        client, addr = sock.accept()
        print(f"[feature_server] Client connected from {addr}")
        with self._lock:
            self._client_socket = client

    def send_json_line(self, obj: Dict[str, Any]) -> None:
        line = json.dumps(obj, separators=(",", ":")) + "\n"
        data = line.encode("utf-8")
        with self._lock:
            if self._client_socket is None:
                return
            try:
                self._client_socket.sendall(data)
            except (BrokenPipeError, ConnectionResetError):
                self._client_socket = None

    def close(self) -> None:
        with self._lock:
            if self._client_socket is not None:
                self._client_socket.close()
                self._client_socket = None
            if self._server_socket is not None:
                self._server_socket.close()
                self._server_socket = None


# -------------------------------------------------------------------------
# Main loop
# -------------------------------------------------------------------------


def run_feature_server(
    registry_path: str,
    logical_device_id: str,
    host: str,
    port: int,
    session_id: str,
    window_seconds: float,
    interval_seconds: float,
) -> None:
    registry = load_device_registry(registry_path)
    device_config = get_logical_device(registry, logical_device_id)

    backend = BrainFlowBackend(device_config)
    backend.start()

    server = NDJSONServer(host, port)
    try:
        server.start()

        prev_horror_state: Dict[str, float] = {}
        while True:
            window = backend.get_window(window_seconds)
            bands = compute_band_powers(window, backend.sampling_rate)
            composite = compute_composite_features(bands)
            horror_context = compute_horror_context(composite, prev_horror_state)
            prev_horror_state = horror_context.copy()

            payload = build_eeg_feature_contract(device_config, bands, composite, horror_context, session_id)
            server.send_json_line(payload)

            time.sleep(interval_seconds)
    finally:
        server.close()
        backend.stop()


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(description="Horror$Place EEG feature server template.")
    parser.add_argument("--registry", required=True, help="Path to EEGDeviceRegistry.json.")
    parser.add_argument("--device-id", required=True, help="Logical device ID from the registry.")
    parser.add_argument("--host", default="127.0.0.1", help="Host to bind the NDJSON TCP server.")
    parser.add_argument("--port", type=int, default=7777, help="Port to bind the NDJSON TCP server.")
    parser.add_argument("--session-id", default="session-001", help="Session identifier for meta.session_id.")
    parser.add_argument("--window-seconds", type=float, default=1.0, help="EEG window length in seconds.")
    parser.add_argument("--interval-seconds", type=float, default=0.5, help="Interval between feature snapshots.")
    return parser.parse_args()


def main() -> None:
    args = parse_args()
    run_feature_server(
        registry_path=args.registry,
        logical_device_id=args.device_id,
        host=args.host,
        port=args.port,
        session_id=args.session_id,
        window_seconds=args.window_seconds,
        interval_seconds=args.interval_seconds,
    )


if __name__ == "__main__":
    main()
