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
