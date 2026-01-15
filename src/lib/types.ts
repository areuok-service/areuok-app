/**
 * Shared types for the areuok application.
 */

/** Daily inspirational quote */
export interface Quote {
  text: string;
  author: string;
}

/** Application view state */
export type View = 'login' | 'dashboard';

/** Dashboard tab view */
export type TabView = 'signin' | 'supervision';

/** Toast message type */
export type ToastType = 'success' | 'error' | 'info' | 'warning';

/** Toast message */
export interface ToastMessage {
  id: number;
  message: string;
  type: ToastType;
  duration: number;
}
