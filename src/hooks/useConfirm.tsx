import { modals } from '@mantine/modals';
import { Text } from '@mantine/core';

interface ConfirmOptions {
  title?: string;
  message: string;
  confirmLabel?: string;
  cancelLabel?: string;
  danger?: boolean;
}

export function useConfirm() {
  const confirm = (options: ConfirmOptions): Promise<boolean> => {
    return new Promise((resolve) => {
      modals.openConfirmModal({
        title: options.title ?? 'Confirm Action',
        children: <Text size="sm">{options.message}</Text>,
        labels: {
          confirm: options.confirmLabel ?? 'Confirm',
          cancel: options.cancelLabel ?? 'Cancel',
        },
        confirmProps: { color: options.danger ? 'red' : 'blue' },
        onConfirm: () => resolve(true),
        onCancel: () => resolve(false),
      });
    });
  };

  return { confirm };
}