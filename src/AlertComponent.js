import React from 'react';
import { Alert, AlertTitle, Collapse, Snackbar } from "@mui/material";

class AlertComponent extends React.Component {
  /**
   * 
   * @param {
   *   status:
   *   message: 
   *   snackbarOn:
   *   handleSnackbarClose: 
   * } props 
   */
  constructor(props) {
    super(props);
  }

  isAlertOn() {
    if (this.props.status === 'offline' || this.props.status === 'connected' || this.props.status === 'recordStopped') {
      return false;
    } else {
      return true;
    }
  }

  getSeverity() {
    if (this.props.status === 'offline') {
        return "success"
    } else if (this.props.status === 'recordStopped') {
        return "info";
    } else if (this.props.status === 'connected') {
        return "success";
    } else if (this.props.status === 'connectionTimeout') {
        return "error";
    } else if (this.props.status === 'pending') {
        return "warning";
    } else {
        return "error";
    }
  }

  getAlertTitle() {
    if (this.props.status === 'offline') {
        return "Success";
    } else if (this.props.status === 'recordStopped') {
        return "Info";
    } else if (this.props.status === 'connected') {
        return "Success";
    } else if (this.props.status === 'connectionTimeout') {
        return "Error";
    } else if (this.props.status === 'pending') {
        return "Loading";
    } else {
        return "Error";
    }
  }

  getAlertMessage() {
    if (this.props.status === 'offline') {
        return "Offline Success";
    } else if (this.props.status === 'recordStopped') {
        // return this.props.message;
        return "Recording stopped!";
    } else if (this.props.status === 'connected') {
        return "Connected with Beckhoff PLC!";
    } else if (this.props.status === 'connectionTimeout') {
        return "Connection Timeout!";
    } else if (this.props.status === 'pending') {
        return "Loading... Please Wait";
    } else {
        return "Unknown Error";
    }
  }

  render() {
    let on = this.isAlertOn();
    let severity = this.getSeverity();
    let alertTitle = this.getAlertTitle();
    let alertMessage = this.getAlertMessage();
    return (
      <div>
        <Collapse in={on}>
        <Alert severity={severity}>
        <AlertTitle>
          {
            // Alert Title
            alertTitle
          }
        </AlertTitle>
          {
            // Alert Content
            alertMessage
          }
        </Alert>
        </Collapse>

        <Snackbar open={this.props.snackbarOn} autoHideDuration={6000} onClose={this.props.handleSnackbarClose} key={new Date().getTime()}>
          <Alert onClose={this.props.handleSnackbarClose} severity={severity}>
            {
              // Snackbar message
              alertMessage
            }
          </Alert>
        </Snackbar>
      </div>
    );
  }
}

export default AlertComponent;