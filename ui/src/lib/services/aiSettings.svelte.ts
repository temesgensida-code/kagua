export interface AIModelOption {
  id: string;
  name: string;
  provider: 'xai' | 'google' | 'openai';
  providerName: string;
}

export const AI_MODELS: AIModelOption[] = [
  // Google Gemini
  { id: 'gemini-2.5-flash', name: 'Gemini 2.5 Flash', provider: 'google', providerName: 'Google' },
  { id: 'gemini-2.5-pro', name: 'Gemini 2.5 Pro', provider: 'google', providerName: 'Google' },
  { id: 'gemini-2.0-flash', name: 'Gemini 2.0 Flash', provider: 'google', providerName: 'Google' },
  // xAI Grok
  { id: 'grok-3', name: 'Grok 3', provider: 'xai', providerName: 'xAI' },
  { id: 'grok-3-mini', name: 'Grok 3 Mini', provider: 'xai', providerName: 'xAI' },
  // OpenAI GPT
  { id: 'gpt-4o', name: 'GPT-4o', provider: 'openai', providerName: 'OpenAI' },
  { id: 'gpt-4o-mini', name: 'GPT-4o Mini', provider: 'openai', providerName: 'OpenAI' },
  { id: 'gpt-4.1', name: 'GPT-4.1', provider: 'openai', providerName: 'OpenAI' },
];

let enabledState = $state(false);
let apiKeyState = $state('');
let selectedModelIdState = $state('gemini-2.5-flash');

const STORAGE_KEY = 'kagua_ai_settings';

if (typeof window !== 'undefined') {
  try {
    const raw = localStorage.getItem(STORAGE_KEY);
    if (raw) {
      const parsed = JSON.parse(raw);
      if (typeof parsed.enabled === 'boolean') enabledState = parsed.enabled;
      if (typeof parsed.apiKey === 'string') apiKeyState = parsed.apiKey;
      if (typeof parsed.selectedModelId === 'string' && AI_MODELS.some(m => m.id === parsed.selectedModelId)) {
        selectedModelIdState = parsed.selectedModelId;
      }
    }
  } catch (e) {
    console.warn('Failed to load AI settings from localStorage:', e);
  }
}

function save() {
  if (typeof window !== 'undefined') {
    try {
      localStorage.setItem(STORAGE_KEY, JSON.stringify({
        enabled: enabledState,
        apiKey: apiKeyState,
        selectedModelId: selectedModelIdState,
      }));
    } catch (e) {
      console.warn('Failed to save AI settings to localStorage:', e);
    }
  }
}

export const aiSettings = {
  get enabled() {
    return enabledState;
  },
  get apiKey() {
    return apiKeyState;
  },
  get selectedModelId() {
    return selectedModelIdState;
  },
  get selectedModel(): AIModelOption {
    return AI_MODELS.find(m => m.id === selectedModelIdState) || AI_MODELS[0];
  },
  get provider() {
    return this.selectedModel.provider;
  },
  setEnabled(val: boolean) {
    enabledState = val;
    save();
  },
  setApiKey(val: string) {
    apiKeyState = val.trim();
    save();
  },
  setSelectedModelId(val: string) {
    if (AI_MODELS.some(m => m.id === val)) {
      selectedModelIdState = val;
      save();
    }
  },
  update(enabled: boolean, apiKey: string, modelId: string) {
    enabledState = enabled;
    apiKeyState = apiKey.trim();
    if (AI_MODELS.some(m => m.id === modelId)) {
      selectedModelIdState = modelId;
    }
    save();
  }
};
