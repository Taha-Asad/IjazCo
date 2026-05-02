import { Routes, Route, Navigate } from "react-router";
import { ProtectedRoute } from "./ProtectedRoute";
import { AppShellLayout } from "../components/layout/AppShell";

// Auth pages
import { LoginPage } from "../pages/auth/LoginPage";
import { RegisterPage } from "../pages/auth/RegisterPage";
import { ForgotPasswordPage } from "../pages/auth/ForgotPasswordPage";
import { ResetPasswordPage } from "../pages/auth/ResetPasswordPage";

// App pages
import { DashboardPage } from "../pages/dashboard/DashboardPage";
import { UsersPage } from "../pages/users/UsersPage";
import { UserDetailPage } from "../pages/users/UserDetailPage";
import { CreateUserPage } from "../pages/users/CreateUserPage";
import { CompaniesPage } from "../pages/companies/CompaniesPage";
import { CompanyDetailPage } from "../pages/companies/CompanyDetailPage";
import { RolesPage } from "../pages/roles/RolesPage";
import { RoleDetailPage } from "../pages/roles/RoleDetailPage";
import { CategoriesPage } from "../pages/categories/CategoriesPage";
import { CustomersPage } from "../pages/customers/CustomersPage";
import { CustomerDetailPage } from "../pages/customers/CustomerDetailPage";
import { SuppliersPage } from "../pages/suppliers/SuppliersPage";
import { SupplierDetailPage } from "../pages/suppliers/SupplierDetailPage";
import { InventoryPage } from "../pages/inventory/InventoryPage";
import { InventoryDetailPage } from "../pages/inventory/InventoryDetailPage";
import { CreateInventoryPage } from "../pages/inventory/CreateInventoryPage";
import { EditInventoryPage } from "../pages/inventory/EditInventoryPage";
import { LowStockPage } from "../pages/inventory/LowStockPage";
import { StockPage } from "../pages/stock/StockPage";
import { StockMovementsPage } from "../pages/stock/StockMovementsPage";
import { PhysicalCountPage } from "../pages/stock/PhysicalCountPage";
import { SalesPage } from "../pages/sales/SalesPage";
import { InvoiceDetailPage } from "../pages/sales/InvoiceDetailPage";
import { CreateInvoicePage } from "../pages/sales/CreateInvoicePage";
import { PurchasesPage } from "../pages/purchases/PurchasesPage";
// import { PurchaseDetailPage } from "../pages/purchases/PurchaseDetailPage";
// import { CreatePurchasePage } from "../pages/purchases/CreatePurchasePage";
// import { ImportsPage } from "../pages/imports/ImportsPage";
// import { ImportDetailPage } from "../pages/imports/ImportDetailPage";
import { ReportsPage } from "../pages/reports/ReportsPage";
import { SettingsPage } from "../pages/settings/SettingsPage";
import { LeadsPage } from "../pages/leads/LeadsPage";
import { LeadDetailPage, EditLeadPage } from "../pages/leads/LeadDetailPage";

export function AppRouter() {
  return (
    <Routes>
      {/* Public Routes */}
      <Route path="/login" element={<LoginPage />} />
      <Route path="/register" element={<RegisterPage />} />
      <Route path="/forgot-password" element={<ForgotPasswordPage />} />
      <Route path="/reset-password" element={<ResetPasswordPage />} />

      {/* Protected Routes */}
      <Route element={<ProtectedRoute />}>
        <Route element={<AppShellLayout />}>
          <Route path="/" element={<Navigate to="/dashboard" replace />} />
          <Route path="/dashboard" element={<DashboardPage />} />

          {/* Users */}
          <Route path="/users" element={<UsersPage />} />
          <Route path="/users/create" element={<CreateUserPage />} />
          <Route path="/users/:id" element={<UserDetailPage />} />

          {/* Companies */}
          <Route path="/companies" element={<CompaniesPage />} />
          <Route path="/companies/:id" element={<CompanyDetailPage />} />

          {/* Roles */}
          <Route path="/roles" element={<RolesPage />} />
          <Route path="/roles/:id" element={<RoleDetailPage />} />

          {/* Categories */}
          <Route path="/categories" element={<CategoriesPage />} />

          {/* Customers */}
          <Route path="/customers" element={<CustomersPage />} />
          <Route path="/customers/:id" element={<CustomerDetailPage />} />

          {/* Suppliers */}
          <Route path="/suppliers" element={<SuppliersPage />} />
          <Route path="/suppliers/:id" element={<SupplierDetailPage />} />

          {/* Inventory */}
          <Route path="/inventory" element={<InventoryPage />} />
          <Route path="/inventory/low-stock" element={<LowStockPage />} />
          <Route path="/inventory/create" element={<CreateInventoryPage />} />
          <Route path="/inventory/:id" element={<InventoryDetailPage />} />
          <Route path="/inventory/:id/edit" element={<EditInventoryPage />} />

          {/* Stock */}
          <Route path="/stock" element={<StockPage />} />
          <Route path="/stock/movements" element={<StockMovementsPage />} />
          <Route path="/stock/physical-count" element={<PhysicalCountPage />} />

          {/* Sales */}
          <Route path="/sales" element={<SalesPage />} />
          <Route path="/sales/create" element={<CreateInvoicePage />} />
          <Route path="/sales/:id" element={<InvoiceDetailPage />} />

          {/* Purchases */}
          <Route path="/purchases" element={<PurchasesPage />} />
          {/* <Route path="/purchases/create" element={<CreatePurchasePage />} />
          <Route path="/purchases/:id" element={<PurchaseDetailPage />} /> */}

          {/* Imports */}
          {/* <Route path="/imports" element={<ImportsPage />} />
          <Route path="/imports/:id" element={<ImportDetailPage />} /> */}

          {/* Reports */}
          <Route path="/reports" element={<ReportsPage />} />

          {/* Leads */}
          <Route path="/leads" element={<LeadsPage />} />
          <Route path="/leads/:id" element={<LeadDetailPage />} />
          <Route path="/leads/:id/edit" element={<EditLeadPage />} />

          {/* Settings */}
          <Route path="/settings" element={<SettingsPage />} />
        </Route>
      </Route>

      <Route path="*" element={<Navigate to="/" replace />} />
    </Routes>
  );
}
