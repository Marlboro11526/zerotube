import React, { Component } from 'react';
import ReactDOM from 'react-dom';
import './index.css';
import LoginResponse from './messages/login';
import * as serviceWorker from './serviceWorker';
import AuthComponent from './auth/auth';

interface AppProperties {

}

interface AppState {
    isLoggedIn?: boolean,
    text?: string,
    token?: string,
    user?: string,
}

class App extends Component<AppProperties, AppState> {
    constructor(props: AppProperties) {
        super(props);

        this.state = {
            text: undefined,
            token: undefined,
            user: undefined,
        };
    }

    componentDidMount() {
        fetch('http://localhost:8081/', {
            method: 'GET',
            credentials: 'include',
        })
            .then(response => response.json() as Promise<LoginResponse>)
            .then(response => {
                if (response !== null && response.username !== null) {
                    console.log('RESPONSE: ');
                    console.log(response);
                    this.setState({
                        isLoggedIn: true,
                        token: response.token,
                        user: response.username,
                    });
                } else {
                    this.setState({
                        isLoggedIn: false
                    });
                }
            })
            .catch((e) => console.log(e));
    }

    showSecret(): void {
        console.log("STATE:");
        console.log(this.state);

        if (this.state.token !== undefined) {
            fetch('http://localhost:8081/web/secret', {
                method: 'GET',
                credentials: 'include',
                headers: new Headers([['authorization', this.state.token]]),
            })
                .then(response => {
                    if (response.ok) {
                        response.json().then(json => {
                            this.setState({
                                text: json
                            })
                        });

                        return;
                    } else {
                        this.setState({
                            text: 'NOT AUTH\'D!!!'
                        });
                    }
                })
                .catch((e) => console.log(e))
        } else {
            this.setState({
                text: 'NOT AUTH\'D!!!'
            });
        }
    }

    handleAuth(response: LoginResponse): void {
        console.log("HANDLING AUTH");

        if (response.username && response.token) {
            this.setState({
                isLoggedIn: true,
                token: response.token,
                user: response.username,
            });
        } else {
            this.setState({
                isLoggedIn: false,
                token: undefined,
                user: undefined,
            });
        }
    }

    render(): JSX.Element {
        if (this.state === null || this.state.isLoggedIn === undefined) {
            return <span id='loader' />
        }

        let loggedInText;

        if (this.state.isLoggedIn) {
            loggedInText = <p>User: {this.state.user}</p>;
        } else {
            loggedInText = <p>Not logged in...</p>
        }

        const isDebug = true;
        let debugZone;

        if (isDebug) {
            debugZone = <>
                <br />
                <br />
                <p>~DEBUG ZONE~</p>
                <p>Is Logged In: {this.state.isLoggedIn ? 'Yes' : 'No'}</p>
                <p>Text: '{this.state.text}'</p>
                <p>User: {this.state.user}</p>
                <p>Token: {this.state.token}</p>
            </>
        }

        console.log('END OF RENDER:');
        console.log(this.state);

        return (
            <div>
                {/* <LoginForm
                    authHandler={(response: AuthResponse) => this.handleAuth(response)}
                    isLoggedIn={this.state.isLoggedIn}
                /> */}
                <AuthComponent
                    authHandler={(response: LoginResponse) => this.handleAuth(response)}
                    isLoggedIn={this.state.isLoggedIn}
                />

                <button onClick={() => this.showSecret()}>Secret</button>
                {loggedInText}
                <p>{this.state.text}</p>
                {debugZone}
            </div>
        )
    }
}

ReactDOM.render(<App />, document.getElementById('root'));

// If you want your app to work offline and load faster, you can change
// unregister() to register() below. Note this comes with some pitfalls.
// Learn more about service workers: https://bit.ly/CRA-PWA
serviceWorker.unregister();
