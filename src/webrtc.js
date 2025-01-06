const peerConnection = new RTCPeerConnection();

peerConnection.onicecandidate = (event) => {
  if (event.candidate) {
    sendSignalingMessage({
      type: "candidate",
      candidate: event.candidate,
      recipient_id: peerId,
    });
  }
};

async function startCall() {
  const offer = await peerConnection.createOffer();
  await peerConnection.setLocalDescription(offer);

  sendSignalingMessage({
    type: "offer",
    sdp: offer.sdp,
    recipient_id: peerId,
  });
}
