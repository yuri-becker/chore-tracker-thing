import { Button, Center, Container } from '@mantine/core'
import { LogIn } from 'react-feather'
import classes from './page.module.css'

const Page = () =>
  <Container fluid={true} className={classes.container}>
    <Center className={classes.loginContainer}>
      <img alt='the beautiful logo for the app chore tracker thing made by yuri and pubbi on github dot com coming to a pc near you someday i think' aria-hidden src='/choretrackerthinglogo.png' />
      <h1>Chore Tracker Thing</h1>
      <p>Please log in to proceed</p>
      <a href="/oidc/login">
        <Button leftSection={<LogIn />}>Login</Button>
      </a>
    </Center>
    <footer className={classes.footer}>
        Created by&nbsp;<a href="https://github.com/yuri-becker">Yuri</a>&nbsp;and&nbsp;<a href="https://github.com/pubbiii">pubbi</a>
    </footer>
  </Container>

export default Page
