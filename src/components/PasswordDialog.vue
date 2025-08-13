<template>
  <div class="modal" :class="{ 'modal-open': show }">
    <div class="modal-box">
      <h3 class="font-bold text-lg flex items-center gap-2">
        <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5 text-warning" fill="none" viewBox="0 0 24 24" stroke="currentColor">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 15v2m-6 4h12a2 2 0 002-2v-6a2 2 0 00-2-2H6a2 2 0 00-2 2v6a2 2 0 002 2zm10-10V7a4 4 0 00-8 0v4h8z" />
        </svg>
        {{ title }}
      </h3>
      
      <div class="py-4">
        <p class="text-base-content/80 mb-4">{{ message }}</p>
        
        <div class="form-control">
          <label class="label">
            <span class="label-text">{{ t('passwordDialog.password') }}</span>
          </label>
          <div class="relative">
            <input 
              ref="passwordInput"
              :type="showPassword ? 'text' : 'password'"
              class="input input-bordered w-full pr-10"
              :class="{ 'input-error': error }"
              v-model="password"
              :placeholder="mode === 'unlock' ? t('passwordDialog.enterPassword') : t('passwordDialog.setPassword')"
              @keyup.enter="handleConfirm"
              @input="clearError"
            />
            <button 
              type="button"
              class="absolute right-3 top-1/2 transform -translate-y-1/2 text-base-content/60 hover:text-base-content"
              @click="togglePasswordVisibility">
              <svg v-if="showPassword" xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13.875 18.825A10.05 10.05 0 0112 19c-4.478 0-8.268-2.943-9.543-7a9.97 9.97 0 011.563-3.029m5.858.908a3 3 0 114.243 4.243M9.878 9.878l4.242 4.242M9.878 9.878L8.464 8.464m1.414 1.414L18.536 18.536" />
              </svg>
              <svg v-else xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 12a3 3 0 11-6 0 3 3 0 016 0z" />
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M2.458 12C3.732 7.943 7.523 5 12 5c4.478 0 8.268 2.943 9.542 7-1.274 4.057-5.064 7-9.542 7-4.477 0-8.268-2.943-9.542-7z" />
              </svg>
            </button>
          </div>
          <label v-if="error" class="label">
            <span class="label-text-alt text-error">{{ error }}</span>
          </label>
        </div>

        <!-- 确认密码输入框（仅在设置密码时显示） -->
        <div v-if="mode === 'encrypt'" class="form-control mt-4">
          <label class="label">
            <span class="label-text">{{ t('passwordDialog.confirmPassword') }}</span>
          </label>
          <div class="relative">
            <input 
              :type="showConfirmPassword ? 'text' : 'password'"
              class="input input-bordered w-full pr-10"
              :class="{ 'input-error': confirmError }"
              v-model="confirmPassword"
              :placeholder="t('passwordDialog.enterPasswordAgain')"
              @keyup.enter="handleConfirm"
              @input="clearConfirmError"
            />
            <button 
              type="button"
              class="absolute right-3 top-1/2 transform -translate-y-1/2 text-base-content/60 hover:text-base-content"
              @click="toggleConfirmPasswordVisibility">
              <svg v-if="showConfirmPassword" xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13.875 18.825A10.05 10.05 0 0112 19c-4.478 0-8.268-2.943-9.543-7a9.97 9.97 0 011.563-3.029m5.858.908a3 3 0 114.243 4.243M9.878 9.878l4.242 4.242M9.878 9.878L8.464 8.464m1.414 1.414L18.536 18.536" />
              </svg>
              <svg v-else xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 12a3 3 0 11-6 0 3 3 0 016 0z" />
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M2.458 12C3.732 7.943 7.523 5 12 5c4.478 0 8.268 2.943 9.542 7-1.274 4.057-5.064 7-9.542 7-4.477 0-8.268-2.943-9.542-7z" />
              </svg>
            </button>
          </div>
          <label v-if="confirmError" class="label">
            <span class="label-text-alt text-error">{{ confirmError }}</span>
          </label>
        </div>

        <!-- 密码强度提示（仅在设置密码时显示） -->
        <div v-if="mode === 'encrypt' && password" class="mt-3">
          <div class="text-sm text-base-content/80 mb-1">{{ t('passwordDialog.passwordStrength') }}:</div>
          <div class="flex gap-1">
            <div 
              v-for="i in 4" 
              :key="i"
              class="flex-1 h-1 rounded"
              :class="getStrengthBarClass(i)"
            ></div>
          </div>
          <div class="text-xs mt-1" :class="getStrengthTextClass()">
            {{ getStrengthText() }}
          </div>
          <div class="text-xs text-base-content/60 mt-1">
            {{ t('passwordDialog.strengthHint') }}
          </div>
        </div>
      </div>

      <div class="modal-action">
        <button class="btn" @click="handleCancel" :disabled="loading">
          {{ t('common.cancel') }}
        </button>
        <button 
          class="btn btn-primary" 
          @click="handleConfirm" 
          :disabled="!canConfirm || loading"
          :class="{ 'loading': loading }">
          {{ mode === 'unlock' ? t('passwordDialog.unlock') : mode === 'encrypt' ? t('passwordDialog.encrypt') : t('passwordDialog.decrypt') }}
        </button>
      </div>
    </div>
    <div class="modal-backdrop" @click="handleCancel"></div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, watch, nextTick, onBeforeUnmount } from 'vue'
import { useI18n } from 'vue-i18n'

const { t } = useI18n()

// 组件属性
const props = defineProps({
  show: {
    type: Boolean,
    default: false
  },
  mode: {
    type: String as () => 'unlock' | 'encrypt' | 'decrypt',
    default: 'unlock'
  },
  title: {
    type: String,
    default: '密码验证'
  },
  message: {
    type: String,
    default: '请输入密码'
  },
  loading: {
    type: Boolean,
    default: false
  }
})

// 组件事件
const emit = defineEmits(['confirm', 'cancel'])

// 状态
const password = ref('')
const confirmPassword = ref('')
const showPassword = ref(false)
const showConfirmPassword = ref(false)
const error = ref('')
const confirmError = ref('')
const passwordInput = ref<HTMLInputElement>()

// 计算属性
const canConfirm = computed(() => {
  if (props.mode === 'unlock') {
    return password.value.length > 0
  } else if (props.mode === 'encrypt') {
    return password.value.length > 0 && 
           confirmPassword.value.length > 0 && 
           password.value === confirmPassword.value
  } else {
    return password.value.length > 0
  }
})

// 密码强度计算
const passwordStrength = computed(() => {
  const pwd = password.value
  if (!pwd) return 0
  
  let score = 0
  // 长度检查
  if (pwd.length >= 8) score += 1
  if (pwd.length >= 12) score += 1
  
  // 复杂度检查
  if (/[a-z]/.test(pwd) && /[A-Z]/.test(pwd)) score += 1
  if (/\d/.test(pwd)) score += 1
  if (/[!@#$%^&*(),.?":{}|<>]/.test(pwd)) score += 1
  
  return Math.min(score, 4)
})

// 方法
function togglePasswordVisibility() {
  showPassword.value = !showPassword.value
}

function toggleConfirmPasswordVisibility() {
  showConfirmPassword.value = !showConfirmPassword.value
}

function clearError() {
  error.value = ''
}

function clearConfirmError() {
  confirmError.value = ''
}

function getStrengthBarClass(index: number) {
  if (passwordStrength.value >= index) {
    if (passwordStrength.value <= 1) return 'bg-error'
    if (passwordStrength.value <= 2) return 'bg-warning'
    if (passwordStrength.value <= 3) return 'bg-info'
    return 'bg-success'
  }
  return 'bg-base-300'
}

function getStrengthTextClass() {
  if (passwordStrength.value <= 1) return 'text-error'
  if (passwordStrength.value <= 2) return 'text-warning'
  if (passwordStrength.value <= 3) return 'text-info'
  return 'text-success'
}

function getStrengthText() {
  if (passwordStrength.value <= 1) return t('passwordDialog.strength.weak')
  if (passwordStrength.value <= 2) return t('passwordDialog.strength.medium')
  if (passwordStrength.value <= 3) return t('passwordDialog.strength.strong')
  return t('passwordDialog.strength.veryStrong')
}

function validatePassword() {
  if (props.mode === 'encrypt') {
    if (password.value !== confirmPassword.value) {
      confirmError.value = t('passwordDialog.passwordsDoNotMatch')
      return false
    }
  }
  return true
}

function handleConfirm() {
  if (!validatePassword()) return
  
  emit('confirm', password.value)
}

function handleCancel() {
  emit('cancel')
  resetForm()
}

function resetForm() {
  password.value = ''
  confirmPassword.value = ''
  showPassword.value = false
  showConfirmPassword.value = false
  error.value = ''
  confirmError.value = ''
}

// 监听显示状态变化
watch(() => props.show, (newShow) => {
  if (newShow) {
    resetForm()
    nextTick(() => {
      passwordInput.value?.focus()
    })
    window.addEventListener('keydown', handleGlobalEnter)
  } else {
    window.removeEventListener('keydown', handleGlobalEnter)
  }
})

// 监听确认密码变化
watch(confirmPassword, () => {
  if (confirmPassword.value && password.value !== confirmPassword.value) {
    confirmError.value = t('passwordDialog.passwordsDoNotMatch')
  } else {
    confirmError.value = ''
  }
})

// Enter 触发确认（避免输入框内重复触发）
function handleGlobalEnter(e: KeyboardEvent) {
  if (e.key !== 'Enter') return
  const target = e.target as HTMLElement | null
  if (target && target.closest('input, textarea, [contenteditable="true"]')) return
  if (!props.show || !canConfirm.value || props.loading) return
  e.preventDefault()
  handleConfirm()
}

onBeforeUnmount(() => {
  window.removeEventListener('keydown', handleGlobalEnter)
})
</script>

<style scoped>
.modal-backdrop {
  background-color: rgba(0, 0, 0, 0.5);
}
</style> 