<template>
    <div class="todo-item" :class="{ completed: props.completed }">
      <input type="checkbox" v-model="localCompleted" @change="updateCompleted" />
      <span>{{ props.title }}</span>
      <button @click="deleteTodo">Delete</button>
    </div>
  </template>
  
<script setup lang="ts">
  import { ref, watch } from 'vue';
  const props = defineProps<{ title: string, completed: boolean }>();
  const emit = defineEmits(['update:completed', 'delete']);
  const localCompleted = ref(props.completed);

  watch(() => props.completed, (newCompleted) => {
    localCompleted.value = newCompleted;
  });

  function updateCompleted() {
    emit('update:completed', localCompleted.value);
  }

  function deleteTodo() {
    emit('delete');
  }
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