package sdk

import (
	"encoding/json"

	"github.com/infisical/go-sdk/internal/cinterface"
)

type InfisicalClientInterface interface {
	Cryptography() CryptographyInterface
	Secrets() SecretsInterface
	Close()
}

type InfisicalClient struct {
	client        cinterface.ClientPointer
	lib           cinterface.InfisicalLibrary
	commandRunner CommandRunnerInterface
	secrets       SecretsInterface
	cryptography  CryptographyInterface
}

func NewInfisicalClient(settings ClientSettings) (InfisicalClientInterface, error) {
	userAgent := "infisical-go-lang-sdk"
	settings.UserAgent = &userAgent

	settingsJSON, err := json.Marshal(settings)
	if err != nil {
		return nil, err
	}

	lib := cinterface.NewInfisicalLibrary()
	client, err := lib.Init(string(settingsJSON))
	if err != nil {
		return nil, err
	}
	runner := NewCommandRunner(client, lib)

	return &InfisicalClient{
		lib:           lib,
		client:        client,
		commandRunner: runner,
		secrets:       NewSecrets(runner),
		cryptography:  NewCryptography(runner),
	}, nil
}

func (c *InfisicalClient) Secrets() SecretsInterface {
	return c.secrets
}

func (c *InfisicalClient) Cryptography() CryptographyInterface {
	return c.cryptography
}

func (c *InfisicalClient) Close() {
	c.lib.FreeMem(c.client)
}
