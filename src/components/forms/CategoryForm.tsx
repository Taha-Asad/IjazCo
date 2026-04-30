import { TextInput, Textarea, Select, Button, Stack } from "@mantine/core";
import { useForm } from "@mantine/form";
import { useQuery } from "@tanstack/react-query";
import { categoriesApi } from "../../api/categories";

interface CategoryFormProps {
  initialValues?: any;
  onSubmit: (values: any) => Promise<void>;
  loading?: boolean;
}

export function CategoryForm({
  initialValues,
  onSubmit,
  loading,
}: CategoryFormProps) {
  const { data } = useQuery({
    queryKey: ["categories-flat"],
    queryFn: () => categoriesApi.list({ per_page: 200 }),
  });

  const parentOptions =
    data?.data
      ?.filter((c) => c.id !== initialValues?.id)
      .map((c) => ({ value: c.id, label: c.name })) || [];

  const form = useForm({
    initialValues: {
      name: initialValues?.name || "",
      description: initialValues?.description || "",
      parent_id: initialValues?.parent_id || null,
    },
    validate: {
      name: (v) => (v.trim().length < 1 ? "Category name required" : null),
    },
  });

  return (
    <form onSubmit={form.onSubmit(onSubmit)}>
      <Stack>
        <TextInput
          label="Category Name"
          placeholder="e.g. Lab Equipment"
          required
          {...form.getInputProps("name")}
        />
        <Select
          label="Parent Category"
          placeholder="Select parent (optional)"
          data={parentOptions}
          clearable
          {...form.getInputProps("parent_id")}
        />
        <Textarea
          label="Description"
          placeholder="Category description"
          rows={3}
          {...form.getInputProps("description")}
        />
        <Button type="submit" loading={loading}>
          {initialValues ? "Save Changes" : "Create Category"}
        </Button>
      </Stack>
    </form>
  );
}
