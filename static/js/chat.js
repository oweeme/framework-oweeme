// Framework Oweeme — Cliente de chat WebSocket
class OweemeChat {
  constructor(room, user, onMessage) {
    this.room = room;
    this.user = user;
    this.onMessage = onMessage;
    this.ws = null;
    this.reconnectDelay = 1000;
    this._connect();
  }

  _connect() {
    const proto = location.protocol === 'https:' ? 'wss' : 'ws';
    const url = `${proto}://${location.host}/ws/chat/${this.room}?user=${encodeURIComponent(this.user)}`;
    this.ws = new WebSocket(url);

    this.ws.addEventListener('open', () => {
      this.reconnectDelay = 1000;
      console.info(`[chat] conectado a sala "${this.room}"`);
    });

    this.ws.addEventListener('message', (e) => {
      try {
        const msg = JSON.parse(e.data);
        this.onMessage(msg);
      } catch (_) {}
    });

    this.ws.addEventListener('close', () => {
      console.warn(`[chat] desconectado, reconectando en ${this.reconnectDelay}ms`);
      setTimeout(() => this._connect(), this.reconnectDelay);
      this.reconnectDelay = Math.min(this.reconnectDelay * 2, 30000);
    });

    this.ws.addEventListener('error', () => this.ws.close());
  }

  send(text) {
    if (this.ws && this.ws.readyState === WebSocket.OPEN) {
      this.ws.send(text.trim());
    }
  }

  close() {
    if (this.ws) this.ws.close();
  }
}

// Ejemplo de uso en una sala:
// const chat = new OweemeChat('artistas-general', 'Héctor', (msg) => {
//   console.log(msg.user, ':', msg.text);
// });
// chat.send('Hola a todos!');

window.OweemeChat = OweemeChat;
