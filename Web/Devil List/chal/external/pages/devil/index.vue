<template>
  <div class="list-container">
    <header class="list-header">
      <h1>Devil List</h1>
    </header>

    <ul class="devil-list">
      <li
        v-for="([name, devil]) in Object.entries(devils)"
        :key="name"
        class="devil-item"
      >
        <div class="devil-info">
          <strong>{{ name }}</strong>
        </div>
        <button @click="view(name)" :id="`devil-${name}`" class="view-btn">
          View
        </button>
      </li>
    </ul>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { navigateTo } from '#app'

type DevilValue = string | { name: string; description?: string }

const devils = ref<Record<string, DevilValue>>({})
const routes = ref<Record<string, string>>({})

onMounted(async () => {
  const devilInfo = await $fetch('/api/devil/list')

  devils.value = devilInfo.devils
  routes.value = devilInfo.devilRoutes
})

const view = async (name: string) => {
  const route = routes.value[name]
  if (route) {
    await navigateTo(route, { external: true })
  }
}
</script>

<style scoped>
.list-container {
  min-height: 100vh;
  background: linear-gradient(to bottom, #0d0d0d, #1a0000);
  color: #f0e6d2;
  font-family: 'Cinzel', serif;
  padding: 40px 20px;
}

.list-header {
  text-align: center;
  margin-bottom: 30px;
}

.list-header h1 {
  font-size: 2.5rem;
  color: #e74c3c;
  text-shadow: 0 0 8px #ff0000;
  border-bottom: 2px solid #800000;
  display: inline-block;
  padding-bottom: 10px;
}

.devil-list {
  list-style: none;
  padding: 0;
  margin: 0 auto;
  max-width: 600px;
}

.devil-item {
  display: flex;
  justify-content: space-between;
  align-items: center;
  background: rgba(20, 0, 0, 0.7);
  border: 1px solid #800000;
  border-radius: 6px;
  padding: 15px 20px;
  margin-bottom: 15px;
  box-shadow: 0 0 12px rgba(255, 0, 0, 0.2);
  transition: transform 0.2s, box-shadow 0.2s;
}

.devil-item:hover {
  transform: translateY(-3px);
  box-shadow: 0 0 16px rgba(255, 0, 0, 0.4);
}

.devil-info strong {
  font-size: 1.2rem;
  color: #f0e6d2;
}

.view-btn {
  background: #800000;
  color: #f0e6d2;
  border: none;
  padding: 8px 16px;
  font-weight: bold;
  border-radius: 4px;
  cursor: pointer;
  transition: background 0.3s, box-shadow 0.3s;
}

.view-btn:hover {
  background: #e74c3c;
  box-shadow: 0 0 8px #ff0000;
}
</style>
