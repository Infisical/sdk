package com.infisical.sdk;

public class InfisicalSettings {

    private String apiUrl;

    private String identityUrl;

    public InfisicalSettings() {
    }

    public InfisicalSettings(String apiUrl, String identityUrl) {
        this.apiUrl = apiUrl;
        this.identityUrl = identityUrl;
    }

    public String getApiUrl() {
        return apiUrl;
    }

    public void setApiUrl(String apiUrl) {
        this.apiUrl = apiUrl;
    }

    public String getIdentityUrl() {
        return identityUrl;
    }

    public void setIdentityUrl(String identityUrl) {
        this.identityUrl = identityUrl;
    }
}
