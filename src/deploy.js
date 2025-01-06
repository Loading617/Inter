const config = {
    iceServers: [
      { urls: "stun:stun.l.google.com:19302" }, // Free STUN server
      { urls: "turn:your-turn-server.com", username: "user", credential: "pass" },
    ],
  };
  const peerConnection = new RTCPeerConnection(config);
  