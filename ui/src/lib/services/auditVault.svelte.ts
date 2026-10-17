import type { AnalysisReport } from './api';

const DB_NAME = 'kagua_vault_v1';
const DB_VERSION = 1;
const AUDITS_STORE = 'audits';
const QUEUE_STORE = 'offline_queue';

function openVault(): Promise<IDBDatabase> {
  return new Promise((resolve, reject) => {
    if (typeof indexedDB === 'undefined') {
      return reject(new Error('IndexedDB not supported in this environment'));
    }

    const request = indexedDB.open(DB_NAME, DB_VERSION);

    request.onupgradeneeded = (event) => {
      const db = (event.target as IDBOpenDBRequest).result;

      if (!db.objectStoreNames.contains(AUDITS_STORE)) {
        const store = db.createObjectStore(AUDITS_STORE, { keyPath: 'local_id' });
        store.createIndex('filename', 'filename', { unique: false });
        store.createIndex('timestamp', 'saved_at', { unique: false });
      }

      if (!db.objectStoreNames.contains(QUEUE_STORE)) {
        const qStore = db.createObjectStore(QUEUE_STORE, { keyPath: 'id', autoIncrement: true });
        qStore.createIndex('queued_at', 'queued_at', { unique: false });
      }
    };

    request.onsuccess = () => resolve(request.result);
    request.onerror = () => reject(request.error);
  });
}

export interface StoredAudit {
  local_id: string;
  saved_at: number;
  report: AnalysisReport;
}

/**
 * Persists an analysis report into the client's local IndexedDB vault.
 */
export async function saveAuditToLocalVault(report: AnalysisReport): Promise<string> {
  try {
    const db = await openVault();
    const local_id = report.id || `local_${Date.now()}_${Math.random().toString(36).substring(2, 9)}`;

    return new Promise((resolve, reject) => {
      const tx = db.transaction(AUDITS_STORE, 'readwrite');
      const store = tx.objectStore(AUDITS_STORE);

      const record: StoredAudit = {
        local_id,
        saved_at: Date.now(),
        report: { ...report, id: local_id },
      };

      const req = store.put(record);
      req.onsuccess = () => resolve(local_id);
      req.onerror = () => reject(req.error);
    });
  } catch (err) {
    console.warn('[AuditVault] Failed to save locally:', err);
    return '';
  }
}

/**
 * Retrieves all stored audits from the local IndexedDB vault.
 */
export async function getAuditsFromLocalVault(): Promise<StoredAudit[]> {
  try {
    const db = await openVault();
    return new Promise((resolve, reject) => {
      const tx = db.transaction(AUDITS_STORE, 'readonly');
      const store = tx.objectStore(AUDITS_STORE);
      const req = store.getAll();

      req.onsuccess = () => {
        const records = (req.result as StoredAudit[]).sort((a, b) => b.saved_at - a.saved_at);
        resolve(records);
      };
      req.onerror = () => reject(req.error);
    });
  } catch (err) {
    console.warn('[AuditVault] Failed to read audits:', err);
    return [];
  }
}

/**
 * Deletes an audit from the local IndexedDB vault by ID.
 */
export async function deleteAuditFromLocalVault(local_id: string): Promise<void> {
  try {
    const db = await openVault();
    return new Promise((resolve, reject) => {
      const tx = db.transaction(AUDITS_STORE, 'readwrite');
      const store = tx.objectStore(AUDITS_STORE);
      const req = store.delete(local_id);

      req.onsuccess = () => resolve();
      req.onerror = () => reject(req.error);
    });
  } catch (err) {
    console.warn('[AuditVault] Failed to delete audit:', err);
  }
}
