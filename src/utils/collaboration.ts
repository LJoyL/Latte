import * as Y from 'yjs';
import { WebsocketProvider } from 'y-websocket';
import { reactive, ref } from 'vue';

export class CollaborationManager {
  private static instance: CollaborationManager;
  public ydoc: Y.Doc;
  public provider: WebsocketProvider | null = null;
  public status = ref<'disconnected' | 'connecting' | 'connected'>('disconnected');
  public awareness: any;
  public currentUser = reactive({
    name: 'User-' + Math.floor(Math.random() * 1000),
    color: '#' + Math.floor(Math.random() * 16777215).toString(16),
  });

  private constructor() {
    this.ydoc = new Y.Doc();
  }

  public static getInstance(): CollaborationManager {
    if (!CollaborationManager.instance) {
      CollaborationManager.instance = new CollaborationManager();
    }
    return CollaborationManager.instance;
  }

  public connect(roomName: string, serverUrl = 'ws://localhost:1234') {
    if (this.provider) {
      this.provider.destroy();
    }

    this.status.value = 'connecting';
    // Using a public demo server for easy testing if local is not available
    // For production, use your own server
    const url = serverUrl || 'wss://demos.yjs.dev'; 
    
    this.provider = new WebsocketProvider(url, roomName, this.ydoc);
    this.awareness = this.provider.awareness;

    this.provider.on('status', (event: any) => {
      this.status.value = event.status; // 'connected' or 'disconnected'
    });

    // Set initial user state
    this.updateUserState();
  }

  public disconnect() {
    if (this.provider) {
      this.provider.destroy();
      this.provider = null;
      this.status.value = 'disconnected';
    }
  }

  public updateUserState(name?: string, color?: string) {
    if (name) this.currentUser.name = name;
    if (color) this.currentUser.color = color;

    if (this.awareness) {
      this.awareness.setLocalStateField('user', {
        name: this.currentUser.name,
        color: this.currentUser.color,
      });
    }
  }
}
