package domain

type Role struct {
	ID        string `gorm:"primaryKey"`
	Name      string
	CreatedAt int64
	UpdatedAt int64
	DeletedAt int64
}
