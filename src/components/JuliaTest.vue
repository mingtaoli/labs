<script setup lang="ts">
import { ref } from "vue";

// 用于显示返回的消息
const pingResponse = ref("");
const addResponse = ref("");
const addPostResponse = ref("");

// 用于输入 x 和 y 值
const xValue = ref(0);
const yValue = ref(0);

// 测试 /ping 路由
async function testPing() {
  try {
    const response = await fetch("http://localhost:19801/ping");
    pingResponse.value = await response.json();
  } catch (error) {
    pingResponse.value = `Error: ${error}`;
  }
}

// 测试 /add/{x}/{y} 路由
async function testAdd() {
  try {
    const response = await fetch(`http://localhost:19801/add/${xValue.value}/${yValue.value}`);
    addResponse.value = await response.text();
  } catch (error) {
    addResponse.value = `Error: ${error}`;
  }
}

// 测试 POST /add_post 路由
async function testAddPost() {
  try {
    const response = await fetch("http://localhost:19801/add_post", {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
      },
      body: JSON.stringify({ x: xValue.value, y: yValue.value }),
    });
    addPostResponse.value = await response.text();
  } catch (error) {
    addPostResponse.value = `Error: ${error}`;
  }
}
</script>

<template>
  <div>
    <h2>测试 Julia 服务</h2>

    <!-- 输入 x 和 y 值 -->
    <div>
      <label for="x">输入 x:</label>
      <input v-model="xValue" type="number" id="x" />
      <label for="y">输入 y:</label>
      <input v-model="yValue" type="number" id="y" />
    </div>

    <!-- Ping 测试 -->
    <button @click="testPing">Ping 测试</button>
    <p>Ping 结果: {{ pingResponse }}</p>

    <!-- Add 测试 -->
    <button @click="testAdd">GET Add ({{ xValue }} + {{ yValue }})</button>
    <p>Add 结果: {{ addResponse }}</p>

    <!-- Add POST 测试 -->
    <button @click="testAddPost">POST Add ({{ xValue }} + {{ yValue }})</button>
    <p>POST Add 结果: {{ addPostResponse }}</p>
  </div>
</template>

<style scoped>
button {
  padding: 0.5em 1em;
  margin: 0.5em;
  border-radius: 8px;
  background-color: #396cd8;
  color: white;
  border: none;
  cursor: pointer;
}

button:hover {
  background-color: #355db2;
}

p {
  font-family: Arial, sans-serif;
  font-size: 1em;
  color: #333;
}

input {
  margin: 0.5em;
  padding: 0.4em;
  border-radius: 4px;
  border: 1px solid #ccc;
  font-size: 1em;
}
</style>
