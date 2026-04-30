import { useState, useEffect } from 'react';
import { Link, useNavigate } from 'react-router-dom';
import { getSuppliers, deleteSupplier, Supplier } from '../services/suppliers';
import './SuppliersPage.css';

export default function SuppliersPage() {
  const navigate = useNavigate();
  const [suppliers, setSuppliers] = useState<Supplier[]>([]);
  const [loading, setLoading] = useState(true);
  const [search, setSearch] = useState('');
  const [deletingId, setDeletingId] = useState<string | null>(null);
  const [error, setError] = useState('');

  const loadSuppliers = async () => {
    try {
      setLoading(true);
      setError('');
      const data = await getSuppliers({ limit: 100 });
      setSuppliers(data.suppliers);
    } catch (err: any) {
      setError(err.message || 'Failed to load suppliers');
    } finally {
      setLoading(false);
    }
  };

  useEffect(() => {
    loadSuppliers();
  }, []);

  const handleDelete = async (id: string) => {
    if (!confirm('Are you sure you want to delete this supplier? This action cannot be undone.')) return;
    try {
      setDeletingId(id);
      await deleteSupplier(id);
      setSuppliers(prev => prev.filter(s => s.id !== id));
    } catch (err: any) {
      alert(err.message || 'Failed to delete supplier');
    } finally {
      setDeletingId(null);
    }
  };

  const filteredSuppliers = suppliers.filter(s => 
    s.name.toLowerCase().includes(search.toLowerCase()) || 
    (s.email && s.email.toLowerCase().includes(search.toLowerCase())) ||
    (s.contact_person && s.contact_person.toLowerCase().includes(search.toLowerCase()))
  );

  return (
    <div className="suppliers-root">
      {/* ── Page Header ── */}
      <div className="suppliers-header">
        <div className="suppliers-header__text">
          <h1 className="suppliers-title">Suppliers</h1>
          <p className="suppliers-subtitle">Manage your product suppliers and vendors.</p>
        </div>
        <div className="suppliers-header__actions">
          <Link to="/suppliers/new" className="suppliers-btn suppliers-btn--primary">
            <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2"><line x1="12" y1="5" x2="12" y2="19"/><line x1="5" y1="12" x2="19" y2="12"/></svg>
            Add New Supplier
          </Link>
        </div>
      </div>

      {error && (
        <div className="suppliers-error" role="alert">
          <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2"><circle cx="12" cy="12" r="10"/><line x1="12" y1="8" x2="12" y2="12"/></svg>
          {error}
        </div>
      )}

      {/* ── Main Card ── */}
      <div className="suppliers-card">
        {/* Card Header (Search) */}
        <div className="suppliers-card__head">
          <h2 className="suppliers-card__title">Supplier Directory</h2>
          <div className="suppliers-search">
            <svg className="suppliers-search__icon" width="15" height="15" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2"><circle cx="11" cy="11" r="8"/><line x1="21" y1="21" x2="16.65" y2="16.65"/></svg>
            <input
              type="search"
              className="suppliers-search__input"
              placeholder="Search by name, contact, email..."
              value={search}
              onChange={(e) => setSearch(e.target.value)}
            />
          </div>
        </div>

        {/* Table */}
        <div className="suppliers-table-wrap">
          <table className="suppliers-table">
            <thead>
              <tr>
                <th>Company Name</th>
                <th>Contact Person</th>
                <th>Contact Info</th>
                <th>Location</th>
                <th>Status</th>
                <th className="suppliers-th--center">Actions</th>
              </tr>
            </thead>
            <tbody>
              {loading && (
                <tr>
                  <td colSpan={6} className="suppliers-table__loading">
                    <div className="suppliers-spinner" />
                    Loading suppliers...
                  </td>
                </tr>
              )}
              
              {!loading && filteredSuppliers.map((supplier) => (
                <tr key={supplier.id}>
                  <td>
                    <div style={{ fontWeight: 600, color: 'var(--c-slate-900)' }}>{supplier.name}</div>
                    <div style={{ fontSize: '0.75rem', color: 'var(--c-slate-400)' }}>Tax ID: {supplier.tax_number || 'N/A'}</div>
                  </td>
                  <td className="suppliers-td--muted">{supplier.contact_person || '-'}</td>
                  <td>
                    <div className="suppliers-td--muted">{supplier.email || '-'}</div>
                    <div className="suppliers-td--muted" style={{ fontSize: '0.75rem' }}>{supplier.phone || '-'}</div>
                  </td>
                  <td className="suppliers-td--muted">
                    {[supplier.city, supplier.country].filter(Boolean).join(', ') || '-'}
                  </td>
                  <td>
                    {supplier.is_active ? (
                      <span className="suppliers-badge suppliers-badge--success">Active</span>
                    ) : (
                      <span className="suppliers-badge suppliers-badge--gray">Inactive</span>
                    )}
                  </td>
                  <td className="suppliers-td--center">
                    <div className="suppliers-actions">
                      <button
                        className="suppliers-action-btn suppliers-action-btn--edit"
                        onClick={() => navigate(`/suppliers/${supplier.id}/edit`)}
                        title="Edit Supplier"
                      >
                        <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2"><path d="M11 4H4a2 2 0 00-2 2v14a2 2 0 002 2h14a2 2 0 002-2v-7"/><path d="M18.5 2.5a2.121 2.121 0 013 3L12 15l-4 1 1-4 9.5-9.5z"/></svg>
                      </button>
                      <button
                        className="suppliers-action-btn suppliers-action-btn--del"
                        onClick={() => handleDelete(supplier.id)}
                        disabled={deletingId === supplier.id}
                        title="Delete Supplier"
                      >
                        <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2"><polyline points="3 6 5 6 21 6"/><path d="M19 6l-1 14H6L5 6"/><path d="M10 11v6"/><path d="M14 11v6"/><path d="M9 6V4h6v2"/></svg>
                      </button>
                    </div>
                  </td>
                </tr>
              ))}

              {!loading && filteredSuppliers.length === 0 && (
                <tr>
                  <td colSpan={6} className="suppliers-table__empty">
                    <svg width="40" height="40" viewBox="0 0 24 24" fill="none" stroke="var(--c-slate-300)" strokeWidth="1.5"><path d="M19 21V5a2 2 0 00-2-2H7a2 2 0 00-2 2v16m14 0h2m-2 0h-5m-9 0H3m2 0h5M9 7h1m-1 4h1m4-4h1m-1 4h1m-5 10v-5a1 1 0 011-1h2a1 1 0 011 1v5m-4 0h4"/></svg>
                    <p>No suppliers found</p>
                    {search ? (
                      <button className="suppliers-empty-link" onClick={() => setSearch('')}>Clear search filter</button>
                    ) : (
                      <Link to="/suppliers/new" className="suppliers-empty-link">Add the first supplier →</Link>
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
