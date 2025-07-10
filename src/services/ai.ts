import { invokeSafe as invoke } from './api'

export const listAIConversations = () => invoke('list_ai_conversations')
export const createAIConversation = (model: string, title: string) => invoke('create_ai_conversation', { model, title }) 