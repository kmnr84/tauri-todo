<template>
    <div class="todo-item" :class="{ completed: completed }">
      <input type="checkbox" v-model="completed" />
      <span>{{ title }}</span>
      <button @click="() => emit('delete')">Remove</button>
    </div>
  </template>
  
<script setup lang="ts">
  const props = defineProps<{ title: string }>();
  const completed = defineModel('completed', { type: Boolean });
  const emit = defineEmits(['delete']);
</script>
  
<style scoped>
  .todo-item {
    display: flex;
    align-items: center;
    margin: 10px 0;
  }
  
  .todo-item.completed span {
    text-decoration: line-through;
    color: gray;
  }
  
  input[type="checkbox"] {
    margin-right: 10px;
  }

  button {
    margin-left: 10px;
    cursor: pointer;
  }
</style>