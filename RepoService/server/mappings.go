package server

import (
	"RepoService/domain"
	"RepoService/request"
	"RepoService/responses"

	"github.com/devfeel/mapper"
)

func InitMapper() {
	mapper.Register(request.CreateSourceRequest{})
	mapper.Register(domain.Source{})
	mapper.Register(responses.Source{})
}
