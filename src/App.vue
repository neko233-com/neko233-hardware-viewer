<template>
  <div class="container">
    <HardwareView />
  </div>
</template>

<script setup lang="ts">
import HardwareView from "./components/HardwareView.vue";
</script>

<style>
body {
  margin: 0;
  padding: 0;
  background-color: #050505; /* Deep black for cyberpunk */
  color: #e0e0e0;
  font-family: 'Segoe UI', Tahoma, Geneva, Verdana, sans-serif;
  overflow-x: hidden;
}
.container {
  width: 100%;
  min-height: 100vh;
}
</style>
