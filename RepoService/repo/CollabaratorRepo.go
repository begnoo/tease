package repo

import (
	"RepoService/domain"
	"fmt"
	"time"

	"gorm.io/gorm"
)

type CollabRepo struct {
	db *gorm.DB
}

func ProvideCollabRepo(db *gorm.DB) CollabRepo {
	return CollabRepo{
		db: db,
	}
}

func (r *CollabRepo) ReadById(id int) (*domain.Collabarator, error) {
	var collabarator domain.Collabarator
	res := r.db.First(&collabarator, id)

	return &collabarator, res.Error
}

func (repo *CollabRepo) ReadBySource(sourceId int) (*[]domain.Collabarator, error) {
	var collabarators []domain.Collabarator
	res := repo.db.Where(&domain.Collabarator{SourceID: sourceId}).Take(&collabarators)

	return &collabarators, res.Error
}

func (r *CollabRepo) Delete(id int) error {
	res := r.db.Delete(&domain.Collabarator{}, id)

	return res.Error
}

func (r *CollabRepo) Update(collabarator domain.Collabarator) error {
	res := r.db.Save(&collabarator)

	return res.Error
}

func (r *CollabRepo) DeleteExpiered() error {
	res := r.db.Where(&domain.Collabarator{}).Where("expieres_at < ?", time.Now()).Delete(&domain.Collabarator{})

	return res.Error
}

func (r *CollabRepo) HandleError(res *gorm.DB) error {
	if res.Error != nil && res.Error != gorm.ErrRecordNotFound {
		err := fmt.Errorf("error: %w", res.Error)
		return err
	}

	return nil
}
