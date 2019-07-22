import React, { Component } from "react";
import UserResponse from "../messages/user";
import ErrorResponse from "../messages/error";
import LoginForm from "./components/login";
import RegisterForm from "./components/register";
import LogoutForm from "./components/logout";
import { toast } from "react-toastify";
import { Button, Buttons } from "trunx";
import { Icon } from "@mdi/react";
import { mdiAccount } from "@mdi/js";

interface AuthProperties {
    authHandler: any,
    isLoggedIn?: boolean,
    username?: string,
}

interface AuthState {
    loginFormInput?: LoginFormInput,
    registerFormInput?: RegisterFormInput,
    showLoginForm: boolean,
    showLogoutConfirmation: boolean,
    showRegisterForm: boolean,
}

interface LoginFormInput {
    password?: string,
    username?: string,
}

interface RegisterFormInput {
    email?: string,
    password?: string,
    username?: string,
}

export default class AuthComponent extends Component<AuthProperties, AuthState> {
    constructor(props: AuthProperties) {
        super(props);

        this.state = {
            showLoginForm: false,
            showLogoutConfirmation: false,
            showRegisterForm: false,
        };
    }

    componentWillMount(): void {
        window.addEventListener("keydown", this.handleKeyDown.bind(this));
    }

    componentWillUnmount(): void {
        window.removeEventListener("keydown", this.handleKeyDown.bind(this));
    }

    handleKeyDown(event: KeyboardEvent): void {
        if (event.key === "Enter") {
            event.preventDefault();

            if (document.activeElement
                && document.activeElement.hasAttribute("data-cancel")) {
                if (this.state.showLoginForm) {
                    this.showLoginForm(false);
                } else if (this.state.showLogoutConfirmation) {
                    this.showLogoutConfirmation(false);
                } else if (this.state.showRegisterForm) {
                    this.showRegisterForm(false);
                }
            } else {
                if (this.state.showLoginForm) {
                    this.handleLogin();
                } else if (this.state.showLogoutConfirmation) {
                    this.handleLogout();
                } else if (this.state.showRegisterForm) {
                    this.handleRegister();
                }
            }
        }
    }

    showLoginForm(isActive: boolean): void {
        this.setState({ showLoginForm: isActive });
    }

    showLogoutConfirmation(isActive: boolean): void {
        this.setState({ showLogoutConfirmation: isActive });
    }

    showRegisterForm(isActive: boolean): void {
        this.setState({ showRegisterForm: isActive });
    }

    handleLoginInput(password?: string, username?: string): void {
        this.setState({
            loginFormInput: {
                password: password,
                username: username,
            }
        });
    }

    handleRegisterInput(email?: string, password?: string, username?: string): void {
        this.setState({
            registerFormInput: {
                email: email,
                password: password,
                username: username,
            }
        });
    }

    async handleLogin(): Promise<void> {
        if (!this.state.loginFormInput
            || !this.state.loginFormInput.password
            || !this.state.loginFormInput.username) {
            toast("Please fill in the form", { type: "warning" });

            return;
        }

        try {
            let response = await this.login(this.state.loginFormInput.username, this.state.loginFormInput.password);
            this.props.authHandler(response);

            this.setState({
                showLoginForm: false
            });
        } catch (error) {
            this.props.authHandler(null);
            toast("Username/password not found.", { type: "error" });
        }
    }

    async handleLogout(): Promise<void> {
        try {
            let response = await this.logout();
            this.props.authHandler(response);

            this.setState({
                showLogoutConfirmation: false
            });
        } catch (error) {
            toast("Logout failed.", { type: "error" });
        }
    }

    async handleRegister(): Promise<void> {
        if (!this.state.registerFormInput
            || !this.state.registerFormInput.email
            || !this.state.registerFormInput.password
            || !this.state.registerFormInput.username) {
            toast("Please fill in the form", { type: "warning" });

            return;
        }

        try {
            await this.register(this.state.registerFormInput.email, this.state.registerFormInput!.username, this.state.registerFormInput.password);

            this.setState({
                showRegisterForm: false
            });
        } catch (error) {
            toast("Registration failed.", { type: "error" });
        }
    }

    async login(username: string, password: string): Promise<UserResponse> {
        return fetch("http://localhost:8081/auth/login", {
            method: "POST",
            credentials: "include",
            headers: new Headers([["Content-Type", "application/json"]]),
            body: JSON.stringify({ username, password })
        })
            .then(async response => {
                if (response.ok) {
                    return response.json();
                } else {
                    let error = await response.json() as ErrorResponse;
                    console.log("Error on login: " + error);

                    throw new Error("Error on login: " + error);
                }
            });
    }

    async logout(): Promise<void> {
        return fetch("http://localhost:8081/auth/logout", {
            method: "POST",
            credentials: "include",
        })
            .then(async response => {
                if (response.ok) {
                    return;
                } else {
                    let error = await response.json() as ErrorResponse;
                    console.log("Error on logout: " + error);

                    throw new Error("Error on logout: " + error);
                }
            });
    }

    async register(email: string, username: string, password: string): Promise<void> {
        console.log("REGISTERING: " + JSON.stringify({ email, username, password }));

        return fetch("http://localhost:8081/auth/register", {
            method: "POST",
            credentials: "include",
            headers: new Headers([["Content-Type", "application/json"]]),
            body: JSON.stringify({ email, username, password })
        })
            .then(async response => {
                if (response.ok) {
                    return;
                } else {
                    let error = await response.json() as ErrorResponse;
                    console.log("Error on registration: " + error);

                    throw new Error("Error on registration: " + error);
                }
            });
    }

    render(): JSX.Element {
        let authForm;
        let authFormButtonCancel;
        let authFormButtonSubmit;
        let authFormModal;
        let authFormName;
        let buttons;
        let username;

        if (this.state.showLoginForm) {
            authForm = <LoginForm inputHandler={(password: string, username: string) => this.handleLoginInput(password, username)} />
            authFormButtonCancel = () => this.showLoginForm(false);
            authFormButtonSubmit = () => this.handleLogin();
            authFormName = "Login";

        } else if (this.state.showRegisterForm) {
            authForm = <RegisterForm inputHandler={(email: string, password: string, username: string) => this.handleRegisterInput(email, password, username)} />
            authFormButtonCancel = () => this.showRegisterForm(false);
            authFormButtonSubmit = () => this.handleRegister();
            authFormName = "Register";

        } else if (this.state.showLogoutConfirmation) {
            authForm = <LogoutForm />
            authFormButtonCancel = () => this.showLogoutConfirmation(false);
            authFormButtonSubmit = () => this.handleLogout();
            authFormName = "Logout";
        }

        if (authForm) {
            authFormModal =
                <div className="modal is-active">
                    <div className="modal-background"></div>
                    <div className="modal-card">
                        <header className="modal-card-head">
                            <p className="modal-card-title">{authFormName}</p>
                            <button className="delete" aria-label="close" onClick={authFormButtonCancel}></button>
                        </header>
                        <section className="modal-card-body">
                            {authForm}
                        </section>
                        <footer className="modal-card-foot">
                            <Button isPrimary onClick={authFormButtonSubmit}>{authFormName}</Button>
                            <Button data-cancel onClick={authFormButtonCancel}>Cancel</Button>
                        </footer>
                    </div>
                </div>
        }

        if (this.props.isLoggedIn) {
            buttons =
                <div className="navbar-item">
                    <Buttons>
                        <Button isPrimary onClick={() => this.showLogoutConfirmation(true)}>
                            Logout
                        </Button>
                    </Buttons>
                </div>

            username =
                <div className="navbar-item">
                    <Icon
                        className="icon-left-offset"
                        color="#FFF"
                        path={mdiAccount}
                        size={"1.5rem"}
                        title="User"
                    />
                    <span>{this.props.username}</span>
                </div >

        } else {
            buttons =
                <div className="navbar-item">
                    <Buttons>
                        <Button isPrimary onClick={() => this.showLoginForm(true)}>
                            Login
                        </Button>
                        <Button isPrimary onClick={() => this.showRegisterForm(true)}>
                            Register
                        </Button>
                    </Buttons>
                </div>
        }

        return (
            <>
                {username}
                {buttons}
                {authFormModal}
            </>
        );
    }
}
