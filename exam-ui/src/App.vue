<script setup>
import { reactive, onMounted, onUnmounted, ref ,nextTick} from "vue";
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";

const seatMap = reactive({});
const logList = ref([]);
const logContainer = ref(null); // 控制滚动条
let unlisten1 = null;
let unlisten2 = null;

function initSeats() {
  for (let i = 0; i < 15; i++) {
    const seatName = `S${i}`;
    // 直接赋值
    seatMap[seatName] = { name: seatName, state: false };
  }
}

async function start_server(){
  try {
    await invoke('server')
    console.log("服务器启动成功")
  } catch (e) {
    console.error("启动失败", e)
  }
}

onMounted(async () => {
  initSeats();
  unlisten1 = await listen('log-output', async (event) =>{
    logList.value.push(event.payload);

    await nextTick();
    if (logContainer.value) {
      logContainer.value.scrollTop = logContainer.value.scrollHeight;
    }
  })
  // 监听 Rust 发来的更新
  unlisten2 = await listen('seats-update', (event) =>{
    console.log('收到更新:', event)
    const data = event.payload; // { name: "S0", state: true }

    // 更新对应的座位
    if (seatMap[data.name]) {
      seatMap[data.name].state = data.state;
    }
  })
})

onUnmounted(() => {
  if (unlisten1) unlisten1();
  if (unlisten2) unlisten2();
})
</script>

<template>
  <main class="container">
    <div class="up_bar">
      <button @click="start_server">启动服务器</button>
      <button >终止服务器</button>
    </div>

    <div class="main-grid">
      <div
          v-for="(seat, key) in seatMap"
          :key="key"
          class="seat-item"
          :class="{ taken: seat.state }"
      >
        <div class="seat-name">{{ seat.name }}</div>
        <div class="seat-status">{{ seat.state ? '已占' : '空闲' }}</div>
      </div>
    </div>

    <div class="log_info" ref = "logContainer">
      <div
          v-for="(log,index) in logList"
          :key = "index"
          class = "log_one"
      >
        <span class="log-time">[{{ log.time }}]</span>
        <span :class="['log-level', log.level]">{{ log.level }}</span>
        <span class="log-msg">{{ log.message }}</span>
    </div>
    </div>
  </main>
</template>

<style scoped>
.main-grid {
  display: grid;
  grid-template-columns: repeat(5, 1fr); /* 每行5个 */
  gap: 15px;
  margin: 20px auto;
  max-width: 600px;
}

.seat-item {
  border: 1px solid #ccc;
  padding: 15px;
  border-radius: 8px;
  background-color: white;
  transition: background-color 0.3s;
}

/* 占用状态变为红色 */
.seat-item.taken {
  background-color: #ff4d4f;
  color: white;
  border-color: #ff4d4f;
}

.seat-name {
  font-weight: bold;
  font-size: 1.2rem;
}

.seat-status {
  font-size: 0.9rem;
}

.up_bar {
  display: flex;
  gap: 10px;
  justify-content: center;
  margin-bottom: 20px;
}

.log_info{
  background-color: #f4f4f4;
  color: #1e1e1e;
  height: 200px; /* 固定高度 */
  overflow-y: auto; /* 超出滚动 */
  padding: 10px;
  border-radius: 8px;
  font-family: 'Consolas', monospace; /* 等宽字体像终端 */
  font-size: 0.9em;
  text-align: left;
}

.log_one {
  margin-bottom: 4px;
}

.log-level { margin-right: 8px; font-weight: bold; }
.log-level.INFO { color: #42b983; }
.log-level.WARN { color: #e6a23c; }
.log-level.ERROR { color: #f56c6c; }
</style>