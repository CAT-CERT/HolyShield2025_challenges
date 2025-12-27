<template>
  <div class="container">
    <nav class="nav">
      <NuxtLink to="/auth/register" class="link">Register</NuxtLink>
      <NuxtLink to="/auth/login" class="link">Login</NuxtLink>
      <NuxtLink to="/devil/" class="link">Devil</NuxtLink>
    </nav>
    <header class="header">
      <h1>Devil List</h1>
    </header>
  </div>
</template>

<style scoped>
.container {
    min-height: 100vh;
    background: radial-gradient(circle at top, #1a1a1a, #000);
    color: #f0e6d2;
    font-family: 'Cinzel', serif;
    display: flex;
    flex-direction: column;
    align-items: center;
    padding-top: 40px;
}

.nav {
    display: flex;
    gap: 20px;
    margin-bottom: 30px;
}

.link {
    color: #c0392b;
    text-decoration: none;
    font-weight: bold;
    transition: color 0.3s, text-shadow 0.3s;
}

.link:hover {
    color: #e74c3c;
    text-shadow: 0 0 8px #ff0000;
}

.header h1 {
    font-size: 3rem;
    letter-spacing: 3px;
    text-transform: uppercase;
    text-shadow: 2px 2px 8px rgba(255, 0, 0, 0.6);
    border-bottom: 2px solid #c0392b;
    padding-bottom: 10px;
}
</style>