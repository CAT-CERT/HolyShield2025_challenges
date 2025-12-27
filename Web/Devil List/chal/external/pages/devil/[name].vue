<template>
  <div class="detail-container">
    <h1>Devil Detail</h1>
    <div v-if="description" class="detail-card">
      <h2>{{ name }}</h2>
      <p>{{ description }}</p>
    </div>
    <div v-else class="loading">Loading...</div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'

const route = useRoute()
const name = route.params.name as string

const description = ref('')

onMounted(async () => {
  description.value = await $fetch(`/api/devil/${name}`)
})
</script>

<style scoped>
:global(body) {
  margin: 0;
  background: #000;
  color: #f0f0f0;
  font-family: "Cinzel", serif;
}

.detail-container {
  max-width: 700px;
  margin: 60px auto;
  padding: 30px;
  text-align: center;
}

.detail-container h1 {
  font-size: 2rem;
  margin-bottom: 20px;
  color: #ff3333;
  text-shadow: 0 0 10px #ff0000;
  letter-spacing: 2px;
}

.detail-card {
  margin-top: 20px;
  padding: 25px;
  border: 1px solid rgba(255, 0, 0, 0.5);
  border-radius: 10px;
  background: rgba(30, 0, 0, 0.85);
  box-shadow: inset 0 0 15px rgba(255, 0, 0, 0.3), 0 0 20px rgba(255, 0, 0, 0.4);
  text-align: left;
}

.detail-card h2 {
  margin-bottom: 12px;
  color: #ff6666;
  font-size: 1.4rem;
  text-shadow: 0 0 6px #ff0000;
}

.detail-card p {
  line-height: 1.6;
  color: #ddd;
}

.loading {
  margin-top: 20px;
  color: #888;
  font-style: italic;
}
</style>
