export type NotificationData = Record<string, unknown> | null

export interface Notification {
  id: string
  user_id: string
  type: string
  title: string
  message: string
  data: NotificationData
  idempotency_key: string | null
  read_at: string | null
  archived_at: string | null
  created_at: string
}

export interface CreateNotificationInput {
  user_id: string
  type: string
  title: string
  message: string
  data?: NotificationData
  idempotency_key?: string
}

export type NotificationSortField = 'created_at' | 'read_at' | 'title' | 'type'
export type NotificationReadStatus = 'all' | 'read' | 'unread'

export interface NotificationListOptions {
  page: number
  pageSize: number
  sortBy?: NotificationSortField
  sortOrder: 'asc' | 'desc'
  includeArchived?: boolean
  readStatus?: NotificationReadStatus
}

export interface NotificationListResult {
  data: Notification[]
  pagination: {
    page: number
    pageSize: number
    total: number
    totalPages: number
    hasNext: boolean
    hasPrev: boolean
  }
  sort: {
    sortBy: NotificationSortField
    sortOrder: 'asc' | 'desc'
  }
}

// User notification preferences for quiet-hours windowing
export interface UserNotificationPreferences {
  id: string
  user_id: string
  timezone: string
  quiet_hours_enabled: boolean
  quiet_hours_start: string
  quiet_hours_end: string
  created_at: string
  updated_at: string
}

export interface UpsertUserNotificationPreferencesInput {
  timezone?: string
  quiet_hours_enabled?: boolean
  quiet_hours_start?: string
  quiet_hours_end?: string
}

// Milestone reminder digest types
export interface MilestoneReminderItem {
  vault_id: string
  milestone_id: string
  milestone_title: string
  due_date: string
  lead_time_ms: number
  lead_time_text: string
}

export interface DigestPayload {
  user_id: string
  items: MilestoneReminderItem[]
  digest_idempotency_key: string
  run_timestamp: string
}

// Deferred reminder for quiet-hours windowing
export interface DeferredReminder {
  id: string
  user_id: string
  idempotency_key: string
  reminder_data: DigestPayload
  deliver_after: string
  created_at: string
}
