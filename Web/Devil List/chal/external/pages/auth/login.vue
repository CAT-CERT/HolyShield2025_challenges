<template>
  <div class="login-container">
    <form class="login-form" @submit.prevent="login">
      <h2>Login</h2>
      <div class="form-group">
        <label>ID</label>
        <input v-model="id" type="text" required />
      </div>
      <div class="form-group">
        <label>PW</label>
        <input v-model="pw" type="password" required />
      </div>
      <div class="form-actions">
        <button type="submit">Enter</button>
      </div>
      <p v-if="msg" class="message">{{ msg }}</p>
    </form>
  </div>
</template>

<script setup>
import { ref } from 'vue';
import { navigateTo } from '#app'

const id = ref('');
const pw = ref('');
const msg = ref('');

const login = async () => {
  try {
    const { message } = await $fetch('/api/auth/login', {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json'
      },
      body: {
        id: id.value,
        pw: pw.value
      }
    });

    msg.value = message;

    if (msg.value === 'success') {
      await navigateTo('/devil/', { external: true })
    }
  } catch (error) {
    msg.value = 'Failed';
  }
};
</script>

<style scoped>
.login-container {
  min-height: 100vh;
  background: linear-gradient(to bottom, #0d0d0d, #1a0000);
  display: flex;
  justify-content: center;
  align-items: center;
  color: #f0e6d2;
  font-family: 'Cinzel', serif;
  padding: 20px;
}

.login-form {
  background: rgba(20, 0, 0, 0.8);
  border: 2px solid #800000;
  border-radius: 8px;
  padding: 30px;
  width: 320px;
  box-shadow: 0 0 20px rgba(255, 0, 0, 0.3);
}

.login-form h2 {
  text-align: center;
  margin-bottom: 20px;
  color: #c0392b;
  text-shadow: 0 0 6px #ff0000;
  font-size: 1.8rem;
  letter-spacing: 2px;
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
  background: #111;
  color: #f0e6d2;
  transition: box-shadow 0.3s, border-color 0.3s;
}

input:focus {
  outline: none;
  border-color: #e74c3c;
  box-shadow: 0 0 8px #ff0000;
}

.form-actions {
  text-align: center;
}

button {
  background: #800000;
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
  color: #ff6666;
  text-shadow: 0 0 6px #ff0000;
}
</style>
