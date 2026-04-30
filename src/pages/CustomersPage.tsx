import { useState, useEffect } from 'react';
import { Link, useNavigate } from 'react-router-dom';
import { getCustomers, deleteCustomer, Customer } from '../services/customers';
import './CustomersPage.css';

export default function CustomersPage() {
  const navigate = useNavigate();
  const [customers, setCustomers] = useState<Customer[]>([]);
  const [loading, setLoading] = useState(true);
  const [search, setSearch] = useState('');
  const [deletingId, setDeletingId] = useState<string | null>(null);
  const [error, setError] = useState('');

  const loadCustomers = async () => {
    try {
      setLoading(true);
      setError('');
      // In a real app we'd pass search params to backend, or do it on frontend
      const data = await getCustomers({ limit: 100 });
      setCustomers(data.customers);
    } catch (err: any) {
      setError(err.message || 'Failed to load customers');
    } finally {
      setLoading(false);
    }
  };

  useEffect(() => {
    loadCustomers();
  }, []);

  const handleDelete = async (id: string) => {
    if (!confirm('Are you sure you want to delete this customer? This action cannot be undone.')) return;
    try {
      setDeletingId(id);
      await deleteCustomer(id);
      setCustomers(prev => prev.filter(c => c.id !== id));
    } catch (err: any) {
      alert(err.message || 'Failed to delete customer');
    } finally {
      setDeletingId(null);
    }
  };

  const filteredCustomers = customers.filter(c => 
    c.name.toLowerCase().includes(search.toLowerCase()) || 
    c.email.toLowerCase().includes(search.toLowerCase()) ||
    (c.phone && c.phone.toLowerCase().includes(search.toLowerCase()))
  );

  return (
    <div className="customers-root">
      {/* ── Page Header ── */}
      <div className="customers-header">
        <div className="customers-header__text">
          <h1 className="customers-title">Customers</h1>
          <p className="customers-subtitle">Manage your customer relationships and credit.</p>
        </div>
        <div className="customers-header__actions">
          <Link to="/customers/new" className="customers-btn customers-btn--primary">
            <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2"><line x1="12" y1="5" x2="12" y2="19"/><line x1="5" y1="12" x2="19" y2="12"/></svg>
            Add New Customer
          </Link>
        </div>
      </div>

      {error && (
        <div className="customers-error" role="alert">
          <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2"><circle cx="12" cy="12" r="10"/><line x1="12" y1="8" x2="12" y2="12"/></svg>
          {error}
        </div>
      )}

      {/* ── Main Card ── */}
      <div className="customers-card">
        {/* Card Header (Search) */}
        <div className="customers-card__head">
          <h2 className="customers-card__title">Customer Directory</h2>
          <div className="customers-search">
            <svg className="customers-search__icon" width="15" height="15" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2"><circle cx="11" cy="11" r="8"/><line x1="21" y1="21" x2="16.65" y2="16.65"/></svg>
            <input
              type="search"
              className="customers-search__input"
              placeholder="Search by name, email..."
              value={search}
              onChange={(e) => setSearch(e.target.value)}
            />
          </div>
        </div>

        {/* Table */}
        <div className="customers-table-wrap">
          <table className="customers-table">
            <thead>
              <tr>
                <th>Customer Name</th>
                <th>Contact Info</th>
                <th>Location</th>
                <th>Credit Limit</th>
                <th>Status</th>
                <th className="customers-th--center">Actions</th>
              </tr>
            </thead>
            <tbody>
              {loading && (
                <tr>
                  <td colSpan={6} className="customers-table__loading">
                    <div className="customers-spinner" />
                    Loading customers...
                  </td>
                </tr>
              )}
              
              {!loading && filteredCustomers.map((customer) => (
                <tr key={customer.id}>
                  <td>
                    <div style={{ fontWeight: 600, color: 'var(--c-slate-900)' }}>{customer.name}</div>
                    <div style={{ fontSize: '0.75rem', color: 'var(--c-slate-400)' }}>Added {new Date(customer.created_at || Date.now()).toLocaleDateString()}</div>
                  </td>
                  <td>
                    <div className="customers-td--muted">{customer.email}</div>
                    <div className="customers-td--muted" style={{ fontSize: '0.75rem' }}>{customer.phone || '-'}</div>
                  </td>
                  <td className="customers-td--muted">
                    {[customer.city, customer.country].filter(Boolean).join(', ') || '-'}
                  </td>
                  <td style={{ fontWeight: 500, color: 'var(--c-slate-900)' }}>
                    ${(customer.credit_limit || 0).toFixed(2)}
                  </td>
                  <td>
                    {customer.is_active ? (
                      <span className="customers-badge customers-badge--success">Active</span>
                    ) : (
                      <span className="customers-badge customers-badge--gray">Inactive</span>
                    )}
                  </td>
                  <td className="customers-td--center">
                    <div className="customers-actions">
                      <button
                        className="customers-action-btn customers-action-btn--edit"
                        onClick={() => navigate(`/customers/${customer.id}/edit`)}
                        title="Edit Customer"
                      >
                        <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2"><path d="M11 4H4a2 2 0 00-2 2v14a2 2 0 002 2h14a2 2 0 002-2v-7"/><path d="M18.5 2.5a2.121 2.121 0 013 3L12 15l-4 1 1-4 9.5-9.5z"/></svg>
                      </button>
                      <button
                        className="customers-action-btn customers-action-btn--del"
                        onClick={() => handleDelete(customer.id)}
                        disabled={deletingId === customer.id}
                        title="Delete Customer"
                      >
                        <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2"><polyline points="3 6 5 6 21 6"/><path d="M19 6l-1 14H6L5 6"/><path d="M10 11v6"/><path d="M14 11v6"/><path d="M9 6V4h6v2"/></svg>
                      </button>
                    </div>
                  </td>
                </tr>
              ))}

              {!loading && filteredCustomers.length === 0 && (
                <tr>
                  <td colSpan={6} className="customers-table__empty">
                    <svg width="40" height="40" viewBox="0 0 24 24" fill="none" stroke="var(--c-slate-300)" strokeWidth="1.5"><path d="M17 21v-2a4 4 0 00-4-4H5a4 4 0 00-4 4v2m8-10a4 4 0 100-8 0 4 4 0 000 8zm6 2v-2a4 4 0 00-3-3.9M19 8a4 4 0 11-8 0 4 4 0 018 0z"/></svg>
                    <p>No customers found</p>
                    {search ? (
                      <button className="customers-empty-link" onClick={() => setSearch('')}>Clear search filter</button>
                    ) : (
                      <Link to="/customers/new" className="customers-empty-link">Add the first customer →</Link>
                    )}
                  </td>
                </tr>
              )}
            </tbody>
          </table>
        </div>
      </div>
      <div style={{ height: 32 }} />
    </div>
  );
}
