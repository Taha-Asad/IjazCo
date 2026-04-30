import { Stack, ThemeIcon, Text, Button } from "@mantine/core";
import { IconInbox } from "@tabler/icons-react";

interface EmptyStateProps {
  title?: string;
  description?: string;
  action?: {
    label: string;
    onClick: () => void;
  };
  icon?: React.ReactNode;
}

export function EmptyState({
  title = "No data found",
  description = "There are no records to display.",
  action,
  icon = <IconInbox size={32} />,
}: EmptyStateProps) {
  return (
    <Stack align="center" py={60} gap="md">
      <ThemeIcon size={64} radius="xl" variant="light" color="gray">
        {icon}
      </ThemeIcon>
      <div style={{ textAlign: "center" }}>
        <Text fw={600} size="lg">
          {title}
        </Text>
        <Text c="dimmed" size="sm" mt={4}>
          {description}
        </Text>
      </div>
      {action && (
        <Button variant="light" onClick={action.onClick}>
          {action.label}
        </Button>
      )}
    </Stack>
  );
}
