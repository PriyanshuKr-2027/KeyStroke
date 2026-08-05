import React, { useEffect, useRef, useState } from 'react';
import { invoke } from '@tauri-apps/api/tauri';
import { listen } from '@tauri-apps/api/event';
import { PaletteContextView } from './PaletteContext';
import { PaletteInputView } from './PaletteInput';
import { PaletteResultView } from './PaletteResult';
import './palette.css';

export interface CapturedContext {
  before: string;
  after: string;
  app_name: string;
  has_text_field: boolean;
}

export type PaletteState =
  | { status: 'idle' }
  | { status: 'loading'; prompt: string }
  | { status: 'result'; text: string }
  | { status: 'error'; message: string };

export const Palette: React.FC = () => {
  const [state, setState] = useState<PaletteState>({ status: 'idle' });
  const [context, setContext] = useState<CapturedContext | null>(null);
  const [prompt, setPrompt] = useState('');
  const inputRef = useRef<HTMLInputElement>(null);

  useEffect(() => {
    const unlisten = listen<CapturedContext>('palette-context', (event) => {
      setContext(event.payload);
      setState({ status: 'idle' });
      setPrompt('');
      setTimeout(() => inputRef.current?.focus(), 50);
    });

    return () => {
      unlisten.then((f) => f());
    };
  }, []);

  useEffect(() => {
    const handleKeyDown = (e: KeyboardEvent) => {
      if (e.key === 'Escape') {
        e.preventDefault();
        invoke('close_palette');
      } else if (e.key === 'Tab' && state.status === 'result') {
        e.preventDefault();
        handleCopy();
      } else if (e.key === 'Enter' && state.status === 'result') {
        e.preventDefault();
        handleInsert();
      }
    };

    window.addEventListener('keydown', handleKeyDown);
    return () => window.removeEventListener('keydown', handleKeyDown);
  }, [state]);

  const handleSubmit = async () => {
    if (!prompt.trim()) return;
    setState({ status: 'loading', prompt });

    try {
      const result = await invoke<string>('run_copilot_prompt', {
        prompt,
        contextBefore: context?.before ?? '',
        contextAfter: context?.after ?? '',
      });
      setState({ status: 'result', text: result });
    } catch (err) {
      setState({ status: 'error', message: String(err) });
    }
  };

  const handleInsert = async () => {
    if (state.status !== 'result') return;
    try {
      await invoke('inject_text', { text: state.text });
    } catch {
      await invoke('copy_to_clipboard', { text: state.text });
    }
  };

  const handleCopy = async () => {
    if (state.status !== 'result') return;
    await invoke('copy_to_clipboard', { text: state.text });
  };

  return (
    <div className="w-screen h-screen flex items-start justify-center p-2 bg-transparent select-none">
      <div className="palette-container w-full max-w-[560px]">
        <PaletteContextView context={context} />

        <PaletteInputView
          prompt={prompt}
          setPrompt={setPrompt}
          onSubmit={handleSubmit}
          isLoading={state.status === 'loading'}
          inputRef={inputRef as React.RefObject<HTMLInputElement>}
        />

        {state.status === 'error' && (
          <div className="px-4 py-2 bg-red-500/10 border-t border-red-500/20 text-red-400 text-xs flex items-center justify-between">
            <span>{state.message}</span>
            <button
              onClick={() => setState({ status: 'idle' })}
              className="text-[10px] underline hover:text-red-300"
            >
              Retry
            </button>
          </div>
        )}

        {state.status === 'result' && (
          <PaletteResultView
            resultText={state.text}
            onInsert={handleInsert}
            onCopy={handleCopy}
          />
        )}
      </div>
    </div>
  );
};
