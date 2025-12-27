<template>
  <div class="archdiocese-container">
    <form class="archdiocese-form" @submit.prevent="archdiocese">
      <h2>Archdiocese Decree</h2>

      <div class="form-group">
        <label>Devil Name</label>
        <input v-model="devilName" type="text" required />
      </div>

      <div class="form-actions">
        <button type="submit">Submit</button>
      </div>

      <p v-if="msg" class="message">{{ msg }}</p>
    </form>
  </div>
</template>

<script setup>
import { ref } from 'vue'
import { navigateTo } from '#app'

const auth = useAuth()

if (!auth.login) {
  await navigateTo('/auth/login', { external: true })
}

const devilName = ref('')
const msg = ref('')

const archdiocese = async () => {
  try {
    const { message } = await $fetch('/api/devil/archdiocese', {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json'
      },
      body: {
        devilName: devilName.value
      }
    })

    msg.value = message
    if (msg.value === 'success') {
      await navigateTo('/devil/', { external: true })
    }
  } catch (error) {
    console.error(error)
    msg.value = 'Failed'
  }
}
</script>

<style scoped>
.archdiocese-container {
  min-height: 100vh;
  background: radial-gradient(circle at center, #1a0000, #000);
  display: flex;
  justify-content: center;
  align-items: center;
  font-family: 'Cinzel', serif;
  color: #f0e6d2;
  padding: 20px;
}

.archdiocese-form {
  background: rgba(30, 0, 0, 0.85);
  border: 2px solid #a93226;
  border-radius: 8px;
  padding: 30px;
  width: 360px;
  box-shadow: 0 0 22px rgba(255, 30, 30, 0.4);
  position: relative;
}

.archdiocese-form::before {
  content: '';
  position: absolute;
  top: -5px;
  left: -5px;
  right: -5px;
  bottom: -5px;
  border: 2px dashed rgba(255, 0, 0, 0.3);
  border-radius: 10px;
  pointer-events: none;
}

.archdiocese-form h2 {
  text-align: center;
  margin-bottom: 20px;
  color: #ff4d4d;
  text-shadow: 0 0 10px #ff0000;
  font-size: 1.6rem;
}

.form-group {
  margin-bottom: 15px;
  display: flex;
  flex-direction: column;
}

label {
  margin-bottom: 6px;
  font-weight: bold;
  font-size: 0.9rem;
}

input {
  padding: 10px;
  border: 1px solid #800000;
  border-radius: 4px;
  background: #f5f0e1;
  color: #1a0000;
  font-family: 'Cinzel', serif;
  transition: box-shadow 0.3s, border-color 0.3s;
}

input:focus {
  outline: none;
  border-color: #e74c3c;
  box-shadow: 0 0 10px #ff0000;
}

.form-actions {
  text-align: center;
}

button {
  background: #b30000;
  color: #f0e6d2;
  border: none;
  padding: 10px 20px;
  font-weight: bold;
  border-radius: 4px;
  cursor: pointer;
  transition: background 0.3s, box-shadow 0.3s;
}

button:hover {
  background: #e74c3c;
  box-shadow: 0 0 12px #ff0000;
}

.message {
  margin-top: 15px;
  text-align: center;
  font-size: 0.95rem;
  color: #ff8080;
  text-shadow: 0 0 6px #ff0000;
}
</style>
