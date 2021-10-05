import React from 'react';
import { Header } from 'semantic-ui-react';
import { Status } from '../../components/status/Status';
import './Decoder.css';

export class Decoder extends React.Component {
  render() {
    return (
      <>
        <Header as="h4" color="blue" textAlign="center">
          Decoder
        </Header>
        <Status />
      </>
    );
  }
}
