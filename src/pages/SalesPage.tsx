import { useState, useEffect } from 'react';
import { Link, useNavigate } from 'react-router-dom';
import { getSalesInvoices, deleteSalesInvoice, SalesInvoice } from '../services/sales';
import './SalesPage.css';

export default function SalesPage() {
  const navigate = useNavigate();
  const [invoices, setInvoices] = useState<SalesInvoice[]>([]);
  const [loading, setLoading] = useState(true);
  const [search, setSearch] = useState('');
  const [statusFilter, setStatusFilter] = useState('all');
  const [deletingId, setDeletingId] = useState<string | null>(null);
  const [error, setError] = useState('');

  const loadInvoices = async () => {
    try {
      setLoading(true);
      setError('');
      // In a real app we'd pass filters to backend
      const data = await getSalesInvoices({ limit: 100 });
      setInvoices(data.invoices);
    } catch (err: any) {
      setError(err.message || 'Failed to load sales invoices');
    } finally {
      setLoading(false);
    }
  };

  useEffect(() => {
    loadInvoices();
  }, []);

  const handleDelete = async (id: string) => {
    if (!confirm('Are you sure you want to delete this invoice?')) return;
    try {
      setDeletingId(id);
      await deleteSalesInvoice(id);
      setInvoices(prev => prev.filter(inv => inv.id !== id));
    } catch (err: any) {
      alert(err.message || 'Failed to delete invoice');
    } finally {
      setDeletingId(null);
    }
  };

  const filteredInvoices = invoices.filter(inv => {
    const matchesSearch = inv.invoice_number.toLowerCase().includes(search.toLowerCase());
    const matchesStatus = statusFilter === 'all' || inv.status === statusFilter;
    return matchesSearch && matchesStatus;
  });

  return (
    <div className="sales-root">
      {/* ── Page Header ── */}
      <div className="sales-header">
        <div className="sales-header__text">
          <h1 className="sales-title">Sales Invoices</h1>
          <p className="sales-subtitle">Manage customer transactions and billing.</p>
        </div>
        <div className="sales-header__actions">
          <Link to="/sales/new" className="sales-btn sales-btn--primary">
            <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2"><line x1="12" y1="5" x2="12" y2="19"/><line x1="5" y1="12" x2="19" y2="12"/></svg>
            Create Invoice
          </Link>
        </div>
      </div>

      {error && (
        <div className="sales-error" role="alert">
          <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2"><circle cx="12" cy="12" r="10"/><line x1="12" y1="8" x2="12" y2="12"/></svg>
          {error}
        </div>
      )}

      {/* ── Main Card ── */}
      <div className="sales-card">
        {/* Card Header (Search & Filter) */}
        <div className="sales-card__head">
          <h2 className="sales-card__title">Invoice Directory</h2>
          
          <div className="sales-filters">
            <div className="sales-search">
              <svg className="sales-search__icon" width="15" height="15" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2"><circle cx="11" cy="11" r="8"/><line x1="21" y1="21" x2="16.65" y2="16.65"/></svg>
              <input
                type="search"
                className="sales-search__input"
                placeholder="Search invoice #..."
                value={search}
                onChange={(e) => setSearch(e.target.value)}
              />
            </div>
            
            <select 
              className="sales-select"
              value={statusFilter}
              onChange={(e) => setStatusFilter(e.target.value)}
            >
              <option value="all">All Statuses</option>
              <option value="draft">Draft</option>
              <option value="pending">Pending</option>
              <option value="approved">Approved</option>
              <option value="paid">Paid</option>
              <option value="cancelled">Cancelled</option>
            </select>
          </div>
        </div>

        {/* Table */}
        <div className="sales-table-wrap">
          <table className="sales-table">
            <thead>
              <tr>
                <th>Invoice #</th>
                <th>Date</th>
                <th>Customer ID</th>
                <th>Status</th>
                <th className="sales-th--right">Total Amount</th>
                <th className="sales-th--center">Actions</th>
              </tr>
            </thead>
            <tbody>
              {loading && (
                <tr>
                  <td colSpan={6} className="sales-table__loading">
                    <div className="sales-spinner" />
                    Loading invoices...
                  </td>
                </tr>
              )}
              
              {!loading && filteredInvoices.map((inv) => (
                <tr key={inv.id}>
                  <td>
                    <div style={{ fontWeight: 600, color: 'var(--c-slate-900)' }}>{inv.invoice_number}</div>
                  </td>
                  <td className="sales-td--muted">
                    {new Date(inv.invoice_date).toLocaleDateString()}
                  </td>
                  <td className="sales-td--muted" style={{ fontSize: '0.75rem' }}>
                    {inv.customer_id.substring(0, 8)}...
                  </td>
                  <td>
                    <span className={`sales-badge sales-badge--${inv.status}`}>{inv.status}</span>
                  </td>
                  <td className="sales-td--right">
                    ${(inv.total_amount || 0).toFixed(2)}
                  </td>
                  <td className="sales-td--center">
                    <div className="sales-actions">
                      <button
                        className="sales-action-btn sales-action-btn--view"
                        onClick={() => navigate(`/sales/${inv.id}`)}
                        title="View Invoice"
                      >
                        <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2"><path d="M1 12s4-8 11-8 11 8 11 8-4 8-11 8-11-8-11-8z"/><circle cx="12" cy="12" r="3"/></svg>
                      </button>
                      <button
                        className="sales-action-btn sales-action-btn--del"
                        onClick={() => handleDelete(inv.id)}
                        disabled={deletingId === inv.id}
                        title="Delete Invoice"
                      >
                        <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2"><polyline points="3 6 5 6 21 6"/><path d="M19 6l-1 14H6L5 6"/><path d="M10 11v6"/><path d="M14 11v6"/><path d="M9 6V4h6v2"/></svg>
                      </button>
                    </div>
                  </td>
                </tr>
              ))}

              {!loading && filteredInvoices.length === 0 && (
                <tr>
                  <td colSpan={6} className="sales-table__empty">
                    <svg width="40" height="40" viewBox="0 0 24 24" fill="none" stroke="var(--c-slate-300)" strokeWidth="1.5"><path d="M14 2H6a2 2 0 00-2 2v16a2 2 0 002 2h12a2 2 0 002-2V8l-6-6z"/><path d="M14 3v5h5M16 13H8M16 17H8M10 9H8"/></svg>
                    <p>No invoices found</p>
                    {search || statusFilter !== 'all' ? (
                      <button className="sales-empty-link" onClick={() => { setSearch(''); setStatusFilter('all'); }}>Clear filters</button>
                    ) : (
                      <Link to="/sales/new" className="sales-empty-link">Create the first invoice →</Link>
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
