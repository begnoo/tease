package server

import (
	"RepoService/domain"
	"RepoService/request"

	"github.com/devfeel/mapper"
)

func InitMapper() {
	mapper.Register(request.CreateSourceRequest{})
	mapper.Register(domain.Source{})
}
