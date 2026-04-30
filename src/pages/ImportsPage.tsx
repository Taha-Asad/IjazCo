import { useState, useEffect } from 'react';
import { Link } from 'react-router-dom';
import { getImports, deleteImport, ImportJob } from '../services/imports';
import { useToast } from '../contexts/ToastContext';
import './ImportsPage.css';

export default function ImportsPage() {
  const [imports, setImports] = useState<ImportJob[]>([]);
  const [loading, setLoading] = useState(true);
  const { success, error } = useToast();

  const loadImports = async () => {
    try {
      setLoading(true);
      const data = await getImports({ limit: 100 });
      setImports(Array.isArray(data.imports) ? data.imports : []);
    } catch (err: any) {
      error(err.message || 'Failed to load imports');
    } finally {
      setLoading(false);
    }
  };

  useEffect(() => {
    loadImports();
  }, []);

  const handleDelete = async (id: string, name: string) => {
    if (!window.confirm(`Are you sure you want to delete the import log for "${name}"?`)) return;
    try {
      await deleteImport(id);
      success('Import log deleted');
      loadImports();
    } catch (err: any) {
      error(err.message || 'Failed to delete import log');
    }
  };

  const formatDate = (dateString: string) => {
    return new Date(dateString).toLocaleString(undefined, {
      year: 'numeric', month: 'short', day: 'numeric',
      hour: '2-digit', minute: '2-digit'
    });
  };

  return (
    <div className="imports-page animate-fade-in">
      <div className="imports-header">
        <div>
          <h1 className="imports-header__title">Data Imports</h1>
          <p className="imports-header__subtitle">Bulk import inventory, customers, and suppliers via CSV.</p>
        </div>
        <Link to="/imports/new" className="btn-primary">
          <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2">
            <path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4"></path>
            <polyline points="17 8 12 3 7 8"></polyline>
            <line x1="12" y1="3" x2="12" y2="15"></line>
          </svg>
          New Import
        </Link>
      </div>

      <div className="imports-content">
        {loading ? (
          <div className="loading-state">Loading import history...</div>
        ) : imports.length === 0 ? (
          <div className="empty-state">
            <svg width="48" height="48" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="1">
              <path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4"></path>
              <polyline points="17 8 12 3 7 8"></polyline>
              <line x1="12" y1="3" x2="12" y2="15"></line>
            </svg>
            <h3>No imports found</h3>
            <p>You haven't imported any data yet.</p>
          </div>
        ) : (
          <div className="imports-table-container">
            <table className="imports-table">
              <thead>
                <tr>
                  <th>File Name</th>
                  <th>Entity Type</th>
                  <th>Status</th>
                  <th>Progress</th>
                  <th>Date</th>
                  <th style={{ textAlign: 'right' }}>Actions</th>
                </tr>
              </thead>
              <tbody>
                {imports.map((job) => (
                  <tr key={job.id}>
                    <td>
                      <div className="import-filename">
                        <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2"><path d="M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8z"></path><polyline points="14 2 14 8 20 8"></polyline><line x1="16" y1="13" x2="8" y2="13"></line><line x1="16" y1="17" x2="8" y2="17"></line><polyline points="10 9 9 9 8 9"></polyline></svg>
                        {job.file_name}
                      </div>
                    </td>
                    <td><span style={{ textTransform: 'capitalize' }}>{job.entity_type}</span></td>
                    <td>
                      <span className={`import-status import-status--${job.status}`}>
                        {job.status}
                      </span>
                    </td>
                    <td>
                      <div className="import-stats">
                        <span title="Total">{job.total_records} records</span>
                        {job.successful_records > 0 && (
                          <span className="import-stat import-stat--success" title="Success">
                            <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2"><polyline points="20 6 9 17 4 12"></polyline></svg>
                            {job.successful_records}
                          </span>
                        )}
                        {job.failed_records > 0 && (
                          <span className="import-stat import-stat--error" title="Failed">
                            <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2"><line x1="18" y1="6" x2="6" y2="18"></line><line x1="6" y1="6" x2="18" y2="18"></line></svg>
                            {job.failed_records}
                          </span>
                        )}
                      </div>
                    </td>
                    <td>{formatDate(job.created_at)}</td>
                    <td>
                      <div style={{ display: 'flex', justifyContent: 'flex-end' }}>
                        <button className="btn-icon btn-icon--danger" onClick={() => handleDelete(job.id, job.file_name)} title="Delete Log">
                          <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2"><polyline points="3 6 5 6 21 6"></polyline><path d="M19 6v14a2 2 0 0 1-2 2H7a2 2 0 0 1-2-2V6m3 0V4a2 2 0 0 1 2-2h4a2 2 0 0 1 2 2v2"></path><line x1="10" y1="11" x2="10" y2="17"></line><line x1="14" y1="11" x2="14" y2="17"></line></svg>
                        </button>
                      </div>
                    </td>
                  </tr>
                ))}
              </tbody>
            </table>
          </div>
        )}
      </div>
    </div>
  );
}
