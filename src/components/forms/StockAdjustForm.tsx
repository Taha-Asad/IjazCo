import {
  NumberInput,
  Textarea,
  Select,
  Button,
  Stack,
  Alert,
} from "@mantine/core";
import { useForm } from "@mantine/form";
import { IconAlertCircle } from "@tabler/icons-react";

const ADJUST_REASONS = [
  { value: "received", label: "Goods Received" },
  { value: "damaged", label: "Damaged / Write-off" },
  { value: "theft", label: "Theft / Loss" },
  { value: "correction", label: "Count Correction" },
  { value: "return", label: "Customer Return" },
  { value: "other", label: "Other" },
];

interface StockAdjustFormProps {
  itemId: string;
  branchId: string;
  currentQuantity: number;
  onSubmit: (values: any) => Promise<void>;
  loading?: boolean;
}

export function StockAdjustForm({
  itemId,
  branchId,
  currentQuantity,
  onSubmit,
  loading,
}: StockAdjustFormProps) {
  const form = useForm({
    initialValues: {
      quantity: 0,
      reason: "",
      notes: "",
    },
    validate: {
      quantity: (v) => (v === 0 ? "Quantity cannot be zero" : null),
      reason: (v) => (!v ? "Reason required" : null),
    },
  });

  const handleSubmit = async (values: typeof form.values) => {
    await onSubmit({ item_id: itemId, branch_id: branchId, ...values });
  };

  const newQty = currentQuantity + form.values.quantity;

  return (
    <form onSubmit={form.onSubmit(handleSubmit)}>
      <Stack>
        {newQty < 0 && (
          <Alert icon={<IconAlertCircle />} color="red" radius="md">
            Adjustment would result in negative stock ({newQty}).
          </Alert>
        )}
        <NumberInput
          label="Adjustment Quantity"
          description={`Current: ${currentQuantity} → New: ${newQty}`}
          placeholder="Use negative to reduce"
          allowNegative
          required
          {...form.getInputProps("quantity")}
        />
        <Select
          label="Reason"
          placeholder="Select reason"
          data={ADJUST_REASONS}
          required
          {...form.getInputProps("reason")}
        />
        <Textarea
          label="Notes"
          placeholder="Additional notes..."
          rows={3}
          {...form.getInputProps("notes")}
        />
        <Button type="submit" loading={loading} disabled={newQty < 0}>
          Apply Adjustment
        </Button>
      </Stack>
    </form>
  );
}
