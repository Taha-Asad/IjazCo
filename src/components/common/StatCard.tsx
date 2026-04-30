import { Paper, Text, Group, ThemeIcon, Badge } from "@mantine/core";
import { IconTrendingUp, IconTrendingDown } from "@tabler/icons-react";

interface StatCardProps {
  title: string;
  value: string | number;
  icon: React.ReactNode;
  change?: number;
  changeLabel?: string;
  color?: string;
  loading?: boolean;
}

export function StatCard({
  title,
  value,
  icon,
  change,
  changeLabel,
  color = "blue",
  loading,
}: StatCardProps) {
  const isPositive = change !== undefined && change >= 0;

  return (
    <Paper p="lg" radius="md" shadow="sm" withBorder>
      <Group justify="space-between" mb="xs">
        <Text size="sm" fw={500} c="dimmed" tt="uppercase">
          {title}
        </Text>
        <ThemeIcon variant="light" color={color} size="lg" radius="md">
          {icon}
        </ThemeIcon>
      </Group>

      <Text size="xl" fw={700} mb="xs">
        {loading ? "..." : value}
      </Text>

      {change !== undefined && (
        <Group gap="xs">
          <Group gap={4}>
            {isPositive ? (
              <IconTrendingUp size={14} color="var(--mantine-color-green-6)" />
            ) : (
              <IconTrendingDown size={14} color="var(--mantine-color-red-6)" />
            )}
            <Text size="xs" c={isPositive ? "green" : "red"} fw={500}>
              {Math.abs(change)}%
            </Text>
          </Group>
          {changeLabel && (
            <Text size="xs" c="dimmed">
              {changeLabel}
            </Text>
          )}
        </Group>
      )}
    </Paper>
  );
}
