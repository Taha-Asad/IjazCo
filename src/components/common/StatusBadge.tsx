import { Badge } from "@mantine/core";

const statusConfig: Record<string, { color: string; label: string }> = {
  draft: { color: "gray", label: "Draft" },
  approved: { color: "blue", label: "Approved" },
  paid: { color: "green", label: "Paid" },
  partial: { color: "yellow", label: "Partial" },
  cancelled: { color: "red", label: "Cancelled" },
  submitted: { color: "cyan", label: "Submitted" },
  received: { color: "green", label: "Received" },
  pending: { color: "orange", label: "Pending" },
  active: { color: "green", label: "Active" },
  inactive: { color: "gray", label: "Inactive" },
  in_transit: { color: "blue", label: "In Transit" },
  delivered: { color: "green", label: "Delivered" },
};

interface StatusBadgeProps {
  status: string;
}

export function StatusBadge({ status }: StatusBadgeProps) {
  const config = statusConfig[status] || { color: "gray", label: status };
  return (
    <Badge color={config.color} variant="light" size="sm">
      {config.label}
    </Badge>
  );
}
