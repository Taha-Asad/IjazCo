import { Group, Title, Text, Button, Breadcrumbs, Anchor } from "@mantine/core";
import { useNavigate } from "react-router-dom";

interface BreadcrumbItem {
  label: string;
  path?: string;
}

interface PageHeaderProps {
  title: string;
  description?: string;
  breadcrumbs?: BreadcrumbItem[];
  action?: {
    label: string;
    icon?: React.ReactNode;
    onClick: () => void;
  };
}

export function PageHeader({
  title,
  description,
  breadcrumbs,
  action,
}: PageHeaderProps) {
  const navigate = useNavigate();

  return (
    <div style={{ marginBottom: 24 }}>
      {breadcrumbs && (
        <Breadcrumbs mb="xs">
          {breadcrumbs.map((item, i) =>
            item.path ? (
              <Anchor
                key={i}
                onClick={() => navigate(item.path!)}
                size="sm"
                style={{ cursor: "pointer" }}
              >
                {item.label}
              </Anchor>
            ) : (
              <Text key={i} size="sm" c="dimmed">
                {item.label}
              </Text>
            ),
          )}
        </Breadcrumbs>
      )}
      <Group justify="space-between" align="flex-start">
        <div>
          <Title order={2}>{title}</Title>
          {description && (
            <Text c="dimmed" mt={4}>
              {description}
            </Text>
          )}
        </div>
        {action && (
          <Button leftSection={action.icon} onClick={action.onClick}>
            {action.label}
          </Button>
        )}
      </Group>
    </div>
  );
}
