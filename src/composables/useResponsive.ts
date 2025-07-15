import { ref, computed } from 'vue'

// Create a reactive reference for the window width.
// This will be shared across all components that use this composable.
const windowWidth = ref(window.innerWidth)

// The resize handler that will be attached to the window.
const onResize = () => {
  windowWidth.value = window.innerWidth
}

let listenerAttached = false

// This composable provides a reactive `isMobile` property.
export function useResponsive() {
  // The event listener is attached only once when the composable is used for the first time.
  // It is attached to the window and will be active for the entire lifecycle of the app.
  // This is suitable for a single-page application where responsiveness is a global concern.
  if (!listenerAttached) {
    window.addEventListener('resize', onResize)
    listenerAttached = true
  }

  // A computed property that determines if the view is mobile based on the window width.
  const isMobile = computed(() => windowWidth.value < 768)

  return {
    isMobile,
  }
} 