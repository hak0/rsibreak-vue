<script setup lang="ts">
import { ref } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { message } from '@tauri-apps/plugin-dialog';



interface Settings {
    work_duration: number
    break_duration: number
    start_time: string
    end_time: string
    active_days: number[]
}

const settings = ref<Settings>({
    work_duration: 45,
    break_duration: 5,
    start_time: '09:00',
    end_time: '18:00',
    active_days: [1, 2, 3, 4, 5]
})

const daysOfWeek = [
    { id: 1, label: '一' },
    { id: 2, label: '二' },
    { id: 3, label: '三' },
    { id: 4, label: '四' },
    { id: 5, label: '五' },
    { id: 6, label: '六' },
    { id: 7, label: '日' },
]

const toggleDay = (dayId: number) => {
    const index = settings.value.active_days.indexOf(dayId)
    if (index === -1) {
        settings.value.active_days.push(dayId)
    } else {
        settings.value.active_days.splice(index, 1)
    }
}

const saveSettings = async () => {
  try {
    await invoke('save_settings', { settings: settings.value })
    await message('设置保存成功', { title: 'Tauri', kind: 'info' });
  } catch (err) {
    await message(`保存失败: ${err}`, { title: 'Tauri', kind: 'error' });
  }
}
</script>

<template>
    <form @submit.prevent="saveSettings" class="space-y-6">
        <!-- 工作时间设置 -->
        <div class="grid grid-cols-2 gap-6">
            <div>
                <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2">
                    工作时间（分钟）
                </label>
                <input v-model.number="settings.work_duration" type="number" min="1" max="120"
                    class="w-full px-4 py-2 rounded-lg border border-gray-300 dark:border-gray-600 focus:ring-2 focus:ring-emerald-500 focus:border-emerald-500 dark:bg-gray-700 dark:text-white">
            </div>

            <div>
                <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2">
                    休息时间（分钟）
                </label>
                <input v-model.number="settings.break_duration" type="number" min="1" max="30"
                    class="w-full px-4 py-2 rounded-lg border border-gray-300 dark:border-gray-600 focus:ring-2 focus:ring-emerald-500 focus:border-emerald-500 dark:bg-gray-700 dark:text-white">
            </div>
        </div>

        <!-- 时间段设置 -->
        <div class="grid grid-cols-2 gap-6">
            <div>
                <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2">
                    开始时间
                </label>
                <input v-model="settings.start_time" type="time" step="300"
                    class="w-full px-4 py-2 rounded-lg border border-gray-300 dark:border-gray-600 focus:ring-2 focus:ring-emerald-500 focus:border-emerald-500 dark:bg-gray-700 dark:text-white">
            </div>

            <div>
                <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2">
                    结束时间
                </label>
                <input v-model="settings.end_time" type="time" step="300"
                    class="w-full px-4 py-2 rounded-lg border border-gray-300 dark:border-gray-600 focus:ring-2 focus:ring-emerald-500 focus:border-emerald-500 dark:bg-gray-700 dark:text-white">
            </div>
        </div>

        <!-- 生效日期 -->
        <div>
            <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-3">
                生效日期
            </label>
            <div class="grid grid-cols-7 gap-2">
                <button v-for="day in daysOfWeek" :key="day.id" type="button" @click="toggleDay(day.id)" :class="[
                    'py-2 rounded transition-colors',
                    settings.active_days.includes(day.id)
                        ? 'bg-emerald-500 text-white hover:bg-emerald-600'
                        : 'bg-gray-100 dark:bg-gray-700 text-gray-700 dark:text-gray-300 hover:bg-gray-200 dark:hover:bg-gray-600'
                ]">
                    {{ day.label }}
                </button>
            </div>
        </div>

        <!-- 保存按钮 -->
        <button type="submit"
            class="w-full py-3 px-6 bg-emerald-600 hover:bg-emerald-700 text-white font-medium rounded-lg transition-colors focus:ring-2 focus:ring-emerald-500 focus:ring-offset-2 dark:focus:ring-offset-gray-900">
            保存设置
        </button>
    </form>
</template>