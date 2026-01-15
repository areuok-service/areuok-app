import {
  isPermissionGranted,
  requestPermission,
  sendNotification,
} from '@tauri-apps/plugin-notification';
import { toastStore } from '../stores/toast';

export type NotificationOptions = {
  title: string;
  body: string;
  icon?: string;
  channelId?: string;
};

class NotificationService {
  #permissionGranted: boolean | null = null;

  async checkPermission(): Promise<boolean> {
    try {
      this.#permissionGranted = await isPermissionGranted();
      return this.#permissionGranted;
    } catch (error) {
      console.error('Failed to check notification permission:', error);
      return false;
    }
  }

  async requestPermission(): Promise<boolean> {
    try {
      const permission = await requestPermission();
      this.#permissionGranted = permission === 'granted';
      return this.#permissionGranted;
    } catch (error) {
      console.error('Failed to request notification permission:', error);
      return false;
    }
  }

  async ensurePermission(): Promise<boolean> {
    if (this.#permissionGranted === true) {
      return true;
    }

    const granted = await this.checkPermission();
    if (granted) {
      return true;
    }

    return await this.requestPermission();
  }

  async send(options: NotificationOptions): Promise<void> {
    const hasPermission = await this.ensurePermission();

    if (!hasPermission) {
      toastStore.warning('通知权限未授予，无法发送系统通知');
      return;
    }

    try {
      sendNotification(options);
    } catch (error) {
      console.error('Failed to send notification:', error);
      toastStore.error('发送通知失败');
    }
  }

  async success(title: string, body: string): Promise<void> {
    await this.send({ title, body });
    toastStore.success(body);
  }

  async error(title: string, body: string): Promise<void> {
    await this.send({ title, body });
    toastStore.error(body);
  }

  async warning(title: string, body: string): Promise<void> {
    await this.send({ title, body });
    toastStore.warning(body);
  }

  async info(title: string, body: string): Promise<void> {
    await this.send({ title, body });
    toastStore.info(body);
  }
}

export const notificationService = new NotificationService();
