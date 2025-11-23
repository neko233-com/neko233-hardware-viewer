import { ref, computed } from 'vue';

interface Task {
  id: string;
  name: string;
  progress: number;
  status: 'running' | 'completed' | 'failed';
  message?: string;
}

const tasks = ref<Task[]>([]);

export function useTasks() {
  const activeTasks = computed(() => tasks.value.filter(t => t.status === 'running'));

  const addTask = (id: string, name: string) => {
    // Remove existing task with same ID if any
    const idx = tasks.value.findIndex(t => t.id === id);
    if (idx !== -1) tasks.value.splice(idx, 1);

    tasks.value.push({
      id,
      name,
      progress: 0,
      status: 'running'
    });
  };

  const updateTask = (id: string, progress: number, status?: 'running' | 'completed' | 'failed', message?: string) => {
    const task = tasks.value.find(t => t.id === id);
    if (task) {
      task.progress = progress;
      if (status) task.status = status;
      if (message) task.message = message;
    }
  };

  const removeTask = (id: string) => {
    const idx = tasks.value.findIndex(t => t.id === id);
    if (idx !== -1) tasks.value.splice(idx, 1);
  };

  return {
    tasks,
    activeTasks,
    addTask,
    updateTask,
    removeTask
  };
}
