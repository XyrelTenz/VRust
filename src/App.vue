<script setup lang="ts">
import { ref, onMounted } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { getCurrentWindow } from "@tauri-apps/api/window";
import { listen } from "@tauri-apps/api/event";

const appWindow = getCurrentWindow();

interface Avd {
  name: string;
}

const avds = ref<Avd[]>([]);
const loading = ref(false);
const error = ref("");
const showDropdown = ref(false);
const currentFrame = ref<string | null>(null);
const isEmulatorRunning = ref(false);

async function fetchAvds() {
  try {
    loading.value = true;
    error.value = "";
    avds.value = await invoke("get_avds");
  } catch (err) {
    error.value = String(err);
  } finally {
    loading.value = false;
  }
}

async function startEmulator(name: string, mode: "normal" | "cold" | "wipe") {
  try {
    showDropdown.value = false;
    isEmulatorRunning.value = true;
    currentFrame.value = null;
    await invoke("launch_emulator", { name, mode });
  } catch (err) {
    alert(`Failed to start: ${err}`);
    isEmulatorRunning.value = false;
  }
}

function toggleDropdown() {
  showDropdown.value = !showDropdown.value;
  if (showDropdown.value) {
    fetchAvds();
  }
}

// Basic interaction commands (placeholders for now)
async function sendKey(key: string) {
  try {
    await invoke("send_key", { key });
  } catch (err) {
    console.error("Failed to send key:", err);
  }
}

const isMouseDown = ref(false);

async function handleMouse(e: MouseEvent, type: 'down' | 'up' | 'move') {
  if (type === 'move' && !isMouseDown.value) return;
  if (type === 'down') isMouseDown.value = true;
  if (type === 'up') isMouseDown.value = false;

  const target = e.currentTarget as HTMLElement;
  const rect = target.getBoundingClientRect();
  
  // Calculate relative X,Y (0-450, 0-800)
  const x = Math.round(((e.clientX - rect.left) / rect.width) * 450);
  const y = Math.round(((e.clientY - rect.top) / rect.height) * 800);
  
  const buttons = isMouseDown.value ? 1 : 0;
  
  try {
    await invoke("send_mouse_event", { x, y, buttons });
  } catch (err) {
    console.error("Mouse event failed:", err);
  }
}

onMounted(() => {
  fetchAvds();
  
  listen<{ data: string }>("emulator-frame", (event) => {
    currentFrame.value = `data:image/png;base64,${event.payload.data}`;
    isEmulatorRunning.value = true;
  });
});
</script>

<template>
  <div
    class="h-screen flex flex-col bg-transparent select-none overflow-hidden rounded-xl border border-white/10 shadow-2xl"
  >
    <!-- Custom Title Bar -->
    <div
      data-tauri-drag-region
      class="h-11 flex items-center justify-between px-4 bg-[#1a1b26]/95 backdrop-blur-2xl border-b border-white/5 z-[60]"
    >
      <div class="flex items-center space-x-3 pointer-events-none">
        <div
          class="w-2 h-2 rounded-full bg-blue-500 animate-pulse shadow-[0_0_8px_rgba(59,130,246,0.5)]"
        ></div>
        <span
          class="text-[10px] font-bold text-gray-400 uppercase tracking-widest"
          >Emulator Runner</span
        >
      </div>

      <div class="flex items-center space-x-1">
        <button
          @click="toggleDropdown"
          class="p-1.5 hover:bg-white/10 rounded-md transition-colors text-gray-300"
          :class="{ 'bg-blue-500/20 text-blue-400': showDropdown }"
        >
          <svg
            xmlns="http://www.w3.org/2000/svg"
            width="16"
            height="16"
            viewBox="0 0 24 24"
            fill="none"
            stroke="currentColor"
            stroke-width="2.5"
            stroke-linecap="round"
            stroke-linejoin="round"
          >
            <line x1="12" y1="5" x2="12" y2="19" />
            <line x1="5" y1="12" x2="19" y2="12" />
          </svg>
        </button>
        <div class="w-px h-3 bg-white/10 mx-1"></div>
        <button
          @click="appWindow.close()"
          class="p-1.5 hover:bg-red-500/20 hover:text-red-400 rounded-md transition-colors text-gray-400"
        >
          <svg
            xmlns="http://www.w3.org/2000/svg"
            width="14"
            height="14"
            viewBox="0 0 24 24"
            fill="none"
            stroke="currentColor"
            stroke-width="2"
            stroke-linecap="round"
            stroke-linejoin="round"
          >
            <line x1="18" y1="6" x2="6" y2="18" />
            <line x1="6" y1="6" x2="18" y2="18" />
          </svg>
        </button>
      </div>
    </div>

    <!-- Dropdown Menu -->
    <Transition name="fade">
      <div v-if="showDropdown" class="absolute top-11 left-2 right-2 z-[70]">
        <div
          class="mt-1 bg-[#1f202e]/98 backdrop-blur-3xl border border-white/10 rounded-xl shadow-2xl overflow-hidden py-2 animate-in slide-in-from-top-2 duration-200"
        >
          <div
            class="px-3 py-1 flex justify-between items-center border-b border-white/5 mb-1 pb-2"
          >
            <span
              class="text-[9px] font-bold text-gray-500 uppercase tracking-widest"
              >Select Device</span
            >
          </div>

          <div v-if="loading" class="px-3 py-6 flex justify-center">
            <div
              class="w-4 h-4 border-2 border-blue-400/30 border-t-blue-400 rounded-full animate-spin"
            ></div>
          </div>

          <div
            v-else-if="avds.length === 0"
            class="px-4 py-4 text-xs text-gray-500 italic text-center"
          >
            No AVDs found
          </div>

          <div v-else class="max-h-64 overflow-y-auto custom-scrollbar">
            <div
              v-for="avd in avds"
              :key="avd.name"
              class="group px-3 py-2 flex items-center justify-between hover:bg-blue-500/10 cursor-pointer transition-colors mx-1 rounded-lg"
              @click="startEmulator(avd.name, 'normal')"
            >
              <div class="flex items-center space-x-3">
                <div
                  class="p-2 bg-blue-500/5 rounded-lg group-hover:bg-blue-500/15 transition-colors"
                >
                  <svg
                    xmlns="http://www.w3.org/2000/svg"
                    width="14"
                    height="14"
                    viewBox="0 0 24 24"
                    fill="none"
                    stroke="currentColor"
                    stroke-width="2"
                    stroke-linecap="round"
                    stroke-linejoin="round"
                    class="text-blue-400/70 group-hover:text-blue-400 transition-colors"
                  >
                    <rect width="14" height="20" x="5" y="2" rx="2" ry="2" />
                    <path d="M12 18h.01" />
                  </svg>
                </div>
                <span
                  class="text-xs text-gray-300 font-medium group-hover:text-blue-100 transition-colors"
                  >{{ avd.name }}</span
                >
              </div>
              <div
                class="flex space-x-1 opacity-0 group-hover:opacity-100 transition-opacity"
              >
                <button
                  @click.stop="startEmulator(avd.name, 'cold')"
                  class="p-1.5 hover:bg-indigo-500/20 rounded text-indigo-400 transition-all"
                  title="Cold Start"
                >
                  <svg
                    xmlns="http://www.w3.org/2000/svg"
                    width="12"
                    height="12"
                    viewBox="0 0 24 24"
                    fill="none"
                    stroke="currentColor"
                    stroke-width="2.5"
                    stroke-linecap="round"
                    stroke-linejoin="round"
                  >
                    <path d="M2 12h10" />
                    <path d="M9 4v16" />
                  </svg>
                </button>
              </div>
            </div>
          </div>
        </div>
      </div>
    </Transition>

    <!-- Main Content Area -->
    <div class="flex-1 flex flex-col bg-[#0d0e14] relative overflow-hidden">
      <!-- Emulator View -->
      <div v-if="isEmulatorRunning" class="flex-1 flex flex-col relative">
        <!-- Action Toolbar (Fixed at Top) -->
        <div
          class="h-12 bg-[#1a1b26]/80 backdrop-blur-md flex items-center justify-center space-x-6 border-b border-white/5 z-20"
        >
          <button
            @click="sendKey('power')"
            class="p-2 hover:bg-white/10 rounded-full text-gray-400 hover:text-red-400 transition-all active:scale-90"
            title="Power"
          >
            <svg
              xmlns="http://www.w3.org/2000/svg"
              width="16"
              height="16"
              viewBox="0 0 24 24"
              fill="none"
              stroke="currentColor"
              stroke-width="2"
              stroke-linecap="round"
              stroke-linejoin="round"
            >
              <path d="M12 2v10" />
              <path d="M18.4 6.6a9 9 0 1 1-12.8 0" />
            </svg>
          </button>
          <div class="w-px h-4 bg-white/5"></div>
          <button
            @click="sendKey('back')"
            class="p-2 hover:bg-white/10 rounded-full text-gray-300 hover:text-blue-400 transition-all active:scale-90"
            title="Back"
          >
            <svg
              xmlns="http://www.w3.org/2000/svg"
              width="18"
              height="18"
              viewBox="0 0 24 24"
              fill="none"
              stroke="currentColor"
              stroke-width="2"
              stroke-linecap="round"
              stroke-linejoin="round"
            >
              <path d="m15 18-6-6 6-6" />
            </svg>
          </button>
          <button
            @click="sendKey('home')"
            class="p-2 hover:bg-white/10 rounded-full text-gray-300 hover:text-blue-400 transition-all active:scale-90"
            title="Home"
          >
            <svg
              xmlns="http://www.w3.org/2000/svg"
              width="18"
              height="18"
              viewBox="0 0 24 24"
              fill="none"
              stroke="currentColor"
              stroke-width="2"
              stroke-linecap="round"
              stroke-linejoin="round"
            >
              <circle cx="12" cy="12" r="10" />
            </svg>
          </button>
          <button
            @click="sendKey('recent')"
            class="p-2 hover:bg-white/10 rounded-full text-gray-300 hover:text-blue-400 transition-all active:scale-90"
            title="Recents"
          >
            <svg
              xmlns="http://www.w3.org/2000/svg"
              width="16"
              height="16"
              viewBox="0 0 24 24"
              fill="none"
              stroke="currentColor"
              stroke-width="2"
              stroke-linecap="round"
              stroke-linejoin="round"
            >
              <rect width="14" height="14" x="5" y="5" rx="2" />
            </svg>
          </button>
        </div>

        <!-- Emulator Screen Container -->
        <div
          class="flex-1 relative group bg-black overflow-hidden cursor-crosshair"
          @mousedown="handleMouse($event, 'down')"
          @mouseup="handleMouse($event, 'up')"
          @mousemove="handleMouse($event, 'move')"
          @mouseleave="handleMouse($event, 'up')"
        >
          <div
            v-if="!currentFrame"
            class="absolute inset-0 flex flex-col items-center justify-center space-y-4 bg-[#0a0a0f] z-10"
          >
            <div
              class="w-8 h-8 border-2 border-blue-400/20 border-t-blue-400 rounded-full animate-spin"
            ></div>
            <span
              class="text-[9px] text-gray-500 uppercase tracking-[0.2em] animate-pulse"
              >Initializing Stream...</span
            >
          </div>

          <img
            v-if="currentFrame"
            :src="currentFrame"
            class="w-full h-full object-contain pointer-events-none"
            draggable="false"
          />

          <!-- Live Overlay -->
          <div
            class="absolute bottom-4 left-4 flex items-center space-x-2 px-2 py-1 bg-black/40 backdrop-blur-md rounded border border-white/10 opacity-0 group-hover:opacity-100 transition-opacity pointer-events-none"
          >
            <div
              class="w-1.5 h-1.5 rounded-full bg-green-500 animate-pulse"
            ></div>
            <span
              class="text-[8px] font-bold text-green-400 uppercase tracking-wider"
              >gRPC Live</span
            >
          </div>
        </div>
      </div>

      <!-- Landing Page -->
      <div
        v-else
        class="flex-1 flex flex-col items-center justify-center text-center p-8 bg-[#0a0b10]"
      >
        <div
          class="absolute top-1/2 left-1/2 -translate-x-1/2 -translate-y-1/2 w-64 h-64 bg-blue-500/10 blur-[80px] rounded-full pointer-events-none"
        ></div>
        <div
          class="relative z-10 opacity-40 hover:opacity-100 transition-opacity duration-700 cursor-default"
        >
          <svg
            width="64"
            height="64"
            viewBox="0 0 24 24"
            fill="none"
            stroke="currentColor"
            stroke-width="1"
            stroke-linecap="round"
            stroke-linejoin="round"
            class="text-gray-500 mb-6 mx-auto"
          >
            <rect width="14" height="20" x="5" y="2" rx="2" ry="2" />
            <path d="M12 18h.01" />
          </svg>
          <p
            class="text-[10px] text-gray-500 leading-relaxed uppercase tracking-[0.2em]"
          >
            Select a device to begin
          </p>
        </div>
      </div>
    </div>
  </div>
</template>

<style>
body {
  margin: 0;
  padding: 0;
  background: transparent !important;
}
#app {
  background: transparent !important;
}
</style>

<style scoped>
.fade-enter-active,
.fade-leave-active {
  transition: all 0.2s ease-out;
}

.fade-enter-from,
.fade-leave-to {
  opacity: 0;
  transform: translateY(-10px);
}

.custom-scrollbar::-webkit-scrollbar {
  width: 4px;
}
.custom-scrollbar::-webkit-scrollbar-track {
  background: transparent;
}
.custom-scrollbar::-webkit-scrollbar-thumb {
  background: rgba(255, 255, 255, 0.1);
  border-radius: 10px;
}
.custom-scrollbar::-webkit-scrollbar-thumb:hover {
  background: rgba(255, 255, 255, 0.2);
}
</style>
