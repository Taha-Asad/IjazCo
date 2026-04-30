import { NumberInput, TextInput, Button, Stack, Select } from "@mantine/core";
import { useForm } from "@mantine/form";

interface StockTransferFormProps {
  itemId: string;
  currentBranchId: string;
  currentQuantity: number;
  branches: { id: string; name: string }[];
  onSubmit: (values: any) => Promise<void>;
  loading?: boolean;
}

export function StockTransferForm({
  branches,
  onSubmit,
  loading,
}: StockTransferFormProps) {
  const form = useForm({
    initialValues: {
      to_branch_id: "",
      quantity: 0,
      notes: "",
    },
    validate: {
      to_branch_id: (v) => (!v ? "Destination branch required" : null),
      quantity: (v) => (v <= 0 ? "Quantity must be greater than 0" : null),
    },
  });

  const branchOptions = branches.map((b) => ({
    value: b.id,
    label: b.name,
  }));

  return (
    <form onSubmit={form.onSubmit(onSubmit)}>
      <Stack>
        <Select
          label="Destination Branch"
          placeholder="Select branch"
          data={branchOptions}
          required
          searchable
          {...form.getInputProps("to_branch_id")}
        />
        <NumberInput
          label="Quantity to Transfer"
          placeholder="Enter quantity"
          required
          min={1}
          {...form.getInputProps("quantity")}
        />
        <TextInput
          label="Notes"
          placeholder="Additional notes (optional)"
          {...form.getInputProps("notes")}
        />
        <Button type="submit" loading={loading}>
          Transfer Stock
        </Button>
      </Stack>
    </form>
  );
}
