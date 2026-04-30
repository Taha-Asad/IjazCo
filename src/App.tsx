import { BrowserRouter as Router, Routes, Route, Navigate } from 'react-router-dom';
import { AuthProvider, useAuth } from './contexts/AuthContext';
import { ThemeProvider } from './contexts/ThemeContext';
import { ToastProvider } from './contexts/ToastContext';
import { ToastContainer } from './components/ToastContainer';
import LoginPage from './pages/LoginPage';
import OtpPage from './pages/OtpPage';
import DashboardPage from './pages/DashboardPage';
import InventoryPage from './pages/InventoryPage';
import SalesPage from './pages/SalesPage';
import PurchasesPage from './pages/PurchasesPage';
import CustomersPage from './pages/CustomersPage';
import SuppliersPage from './pages/SuppliersPage';
import UsersPage from './pages/UsersPage';
import NewSalePage from './pages/NewSalePage';
import NewPurchasePage from './pages/NewPurchasePage';
import NewItemPage from './pages/NewItemPage';
import EditInventoryPage from './pages/EditInventoryPage';
import NewCustomerPage from './pages/NewCustomerPage';
import EditCustomerPage from './pages/EditCustomerPage';
import NewSupplierPage from './pages/NewSupplierPage';
import NewUserPage from './pages/NewUserPage';
import EditSupplierPage from './pages/EditSupplierPage';
import EditUserPage from './pages/EditUserPage';
import CategoriesPage from './pages/CategoriesPage';
import ImportsPage from './pages/ImportsPage';
import NewImportPage from './pages/NewImportPage';
import ReportsPage from './pages/ReportsPage';
import Layout from './components/Layout';

function PrivateRoute({ children }: { children: React.ReactNode }) {
  const { isAuthenticated, loading } = useAuth();

  if (loading) {
    return <div className="flex items-center justify-center min-h-screen">Loading...</div>;
  }

  return isAuthenticated ? <>{children}</> : <Navigate to="/login" />;
}

function PublicRoute({ children }: { children: React.ReactNode }) {
  const { isAuthenticated, loading } = useAuth();

  if (loading) {
    return <div className="flex items-center justify-center min-h-screen">Loading...</div>;
  }

  return !isAuthenticated ? <>{children}</> : <Navigate to="/dashboard" />;
}

function App() {
  return (
    <ThemeProvider>
      <ToastProvider>
        <AuthProvider>
          <Router>
            <Routes>
              <Route path="/login" element={<PublicRoute><LoginPage /></PublicRoute>} />
              <Route path="/otp" element={<PublicRoute><OtpPage /></PublicRoute>} />
              <Route path="/" element={<PrivateRoute><Layout /></PrivateRoute>}>
                <Route index element={<Navigate to="/dashboard" />} />
                <Route path="dashboard" element={<DashboardPage />} />
                <Route path="inventory" element={<InventoryPage />} />
                <Route path="inventory/new" element={<NewItemPage />} />
                <Route path="inventory/:id/edit" element={<EditInventoryPage />} />
                <Route path="sales" element={<SalesPage />} />
                <Route path="sales/new" element={<NewSalePage />} />
                <Route path="categories" element={<CategoriesPage />} />
                <Route path="purchases" element={<PurchasesPage />} />
                <Route path="purchases/new" element={<NewPurchasePage />} />
                <Route path="customers" element={<CustomersPage />} />
                <Route path="customers/new" element={<NewCustomerPage />} />
                <Route path="customers/:id/edit" element={<EditCustomerPage />} />
                <Route path="suppliers" element={<SuppliersPage />} />
                <Route path="suppliers/new" element={<NewSupplierPage />} />
                <Route path="suppliers/:id/edit" element={<EditSupplierPage />} />
                <Route path="users" element={<UsersPage />} />
                <Route path="users/new" element={<NewUserPage />} />
                <Route path="users/:id/edit" element={<EditUserPage />} />
                <Route path="imports" element={<ImportsPage />} />
                <Route path="imports/new" element={<NewImportPage />} />
                <Route path="reports" element={<ReportsPage />} />
              </Route>
            </Routes>
            <ToastContainer />
          </Router>
        </AuthProvider>
      </ToastProvider>
    </ThemeProvider>
  );
}

export default App;
