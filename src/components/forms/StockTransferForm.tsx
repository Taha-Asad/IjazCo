import {
  NumberInput,
  Textarea,
  Select,
  Button,
  Stack,
  Alert,
} from "@mantine/core";
import { useForm } from "@mantine/form";
import { IconArrowRight } from "@tabler/icons-react";

interface Branch {
  id: string;
  name: string;
}

interface StockTransferFormProps {
  itemId: string;
  currentBranchId: string;
  currentQuantity: number;
  branches: Branch[];
  onSubmit: (values: any) => Promise<void>;
  loading?: boolean;
}

export function StockTransferForm({
  itemId,
  currentBranchId,
  currentQuantity,
  branches,
  onSubmit,
  loading,
}: StockTransferFormProps) {
  const form = useForm({
    initialValues: {
      to_branch_id: "",
      quantity: 1,
      notes: "",
    },
    validate: {
      to_branch_id: (v) => (!v ? "Target branch required" : null),
      quantity: (v) =>
        v <= 0
          ? "Quantity must be > 0"
          : v > currentQuantity
            ? "Exceeds available stock"
            : null,
    },
  });

  const targetBranches = branches
    .filter((b) => b.id !== currentBranchId)
    .map((b) => ({ value: b.id, label: b.name }));

  const handleSubmit = async (values: typeof form.values) => {
    await onSubmit({
      item_id: itemId,
      from_branch_id: currentBranchId,
      ...values,
    });
  };

  return (
    <form onSubmit={form.onSubmit(handleSubmit)}>
      <Stack>
        <Select
          label="Transfer To"
          placeholder="Select destination branch"
          data={targetBranches}
          required
          leftSection={<IconArrowRight size={16} />}
          {...form.getInputProps("to_branch_id")}
        />
        <NumberInput
          label="Quantity to Transfer"
          description={`Available: ${currentQuantity}`}
          min={1}
          max={currentQuantity}
          required
          {...form.getInputProps("quantity")}
        />
        <Textarea
          label="Notes"
          placeholder="Transfer reason / notes..."
          rows={3}
          {...form.getInputProps("notes")}
        />
        <Button type="submit" loading={loading}>
          Transfer Stock
        </Button>
      </Stack>
    </form>
  );
}
